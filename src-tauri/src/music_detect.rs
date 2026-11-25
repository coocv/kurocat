//! Ultra-light "is the Mac busy playing music right now?" watcher.
//!
//! Uses CoreAudio's `kAudioDevicePropertyDeviceIsRunningSomewhere` flag on the current default
//! output device: it is a read-only hardware state bit, no audio is recorded or inspected, no
//! permissions are requested, and the whole check costs a couple of microseconds per tick.
//!
//! Heuristics that make it feel musical rather than twitchy:
//! - ~1s poll, requires 6 consecutive "audio running" samples (~6s) before turning ON, so short
//!   key-click blips, notification dings and one-shot UI sounds never trigger it;
//! - 5 consecutive silent samples (~5s) before turning OFF, so pauses between tracks don't flicker
//!   the headphones off and back on.
//!
//! Emits `kuro-music-active` to the frontend; `get_music_active` returns the cached state.

use crate::config::AppSettings;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

static MUSIC_ACTIVE: AtomicBool = AtomicBool::new(false);
/// Set while the pet itself is producing sound (purr), so our own WebAudio stream
/// is never mistaken for a playing song.
static SELF_AUDIO: AtomicBool = AtomicBool::new(false);

/// Marks the pet's own purr as active, so the watcher skips its own audio stream.
pub fn set_self_audio(active: bool) {
    SELF_AUDIO.store(active, Ordering::Relaxed);
}

pub fn is_music_active() -> bool {
    MUSIC_ACTIVE.load(Ordering::Relaxed)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicActivePayload {
    pub active: bool,
}

#[cfg(target_os = "macos")]
mod mac_impl {
    use std::ffi::c_void;
    use std::os::raw::c_int;
    use std::ptr;

    #[repr(C)]
    struct AudioObjectPropertyAddress {
        selector: u32,
        scope: u32,
        element: u32,
    }

    #[link(name = "CoreAudio", kind = "framework")]
    extern "C" {
        fn AudioObjectGetPropertyData(
            object_id: u32,
            in_address: *const AudioObjectPropertyAddress,
            in_qualifier_data_size: u32,
            in_qualifier_data: *const c_void,
            io_data_size: *mut u32,
            out_data: *mut c_void,
        ) -> c_int;
    }

    // CoreAudio four-char codes, big-endian
    const SYSTEM_OBJECT: u32 = 1;
    const SCOPE_GLOBAL: u32 = 0x676C_6F62; // 'glob'
    const ELEMENT_MAIN: u32 = 0;
    const SELECTOR_DEFAULT_OUTPUT: u32 = 0x644F_7574; // 'dOut'
    const SELECTOR_RUNNING_SOMEWHERE: u32 = 0x676F_6E65; // 'gone'

    fn default_output_device() -> Option<u32> {
        unsafe {
            let addr = AudioObjectPropertyAddress {
                selector: SELECTOR_DEFAULT_OUTPUT,
                scope: SCOPE_GLOBAL,
                element: ELEMENT_MAIN,
            };
            let mut device: u32 = 0;
            let mut size = std::mem::size_of::<u32>() as u32;
            let status = AudioObjectGetPropertyData(
                SYSTEM_OBJECT,
                &addr,
                0,
                ptr::null(),
                &mut size,
                &mut device as *mut u32 as *mut c_void,
            );
            if status == 0 && device != 0 {
                Some(device)
            } else {
                None
            }
        }
    }

    /// True while *any* process is rendering audio to the default output device.
    pub fn is_output_playing() -> bool {
        let Some(device) = default_output_device() else {
            return false;
        };
        unsafe {
            let addr = AudioObjectPropertyAddress {
                selector: SELECTOR_RUNNING_SOMEWHERE,
                scope: SCOPE_GLOBAL,
                element: ELEMENT_MAIN,
            };
            let mut running: u32 = 0;
            let mut size = std::mem::size_of::<u32>() as u32;
            let status = AudioObjectGetPropertyData(
                device,
                &addr,
                0,
                ptr::null(),
                &mut size,
                &mut running as *mut u32 as *mut c_void,
            );
            status == 0 && running != 0
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod mac_impl {
    /// Windows: no dependency-free equivalent yet, so music mode stays off on that platform.
    pub fn is_output_playing() -> bool {
        false
    }
}

/// The single answer the watcher asks for each tick.
fn is_music_playing() -> bool {
    mac_impl::is_output_playing()
}

pub fn start_music_detection_watcher(
    app: AppHandle,
    _settings: Arc<Mutex<AppSettings>>,
    paused_guard: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        let mut active = false;
        let mut on_streak: u8 = 0;
        let mut off_streak: u8 = 0;
        // ~6s of continuous audio before the headphones come on
        const ON_TICKS: u8 = 6;
        // ~5s of silence before they come off
        const OFF_TICKS: u8 = 5;

        loop {
            std::thread::sleep(Duration::from_millis(1000));

            if paused_guard.load(Ordering::Relaxed) {
                on_streak = 0;
                off_streak = 0;
                if active {
                    active = false;
                    MUSIC_ACTIVE.store(false, Ordering::Relaxed);
                    let _ = app.emit("kuro-music-active", MusicActivePayload { active: false });
                }
                continue;
            }

            if SELF_AUDIO.load(Ordering::Relaxed) {
                // Our own purr keeps the output device busy; ignore it entirely.
                on_streak = 0;
                off_streak = 0;
                continue;
            }

            if is_music_playing() {
                off_streak = 0;
                on_streak = on_streak.saturating_add(1);
                if !active && on_streak >= ON_TICKS {
                    active = true;
                    MUSIC_ACTIVE.store(true, Ordering::Relaxed);
                    let _ = app.emit("kuro-music-active", MusicActivePayload { active: true });
                }
            } else {
                on_streak = 0;
                off_streak = off_streak.saturating_add(1);
                if active && off_streak >= OFF_TICKS {
                    active = false;
                    MUSIC_ACTIVE.store(false, Ordering::Relaxed);
                    let _ = app.emit("kuro-music-active", MusicActivePayload { active: false });
                }
            }
        }
    });
}
