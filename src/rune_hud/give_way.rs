//! Labs › Cleanup progress (macOS): while its plate shows, the HUD gives way
//! to it unless a capture records, and the plate asks where the HUD is so it
//! can take its place or make room below it.

use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "macos")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "macos")]
use rune::OverlayHandle;

#[cfg(target_os = "macos")]
use super::Hud;

/// Set while the cleanup plate shows.
static GIVE_WAY: AtomicBool = AtomicBool::new(false);
/// Whether the HUD's overlay is on screen.
static HUD_SHOWN: AtomicBool = AtomicBool::new(false);
#[cfg(target_os = "macos")]
static HUD: OnceLock<OverlayHandle<Hud>> = OnceLock::new();
/// The HUD's plate centre in global top-left points, as of its last show
/// below or in the menu bar; `None` in a notch island or set to Hidden.
#[cfg(target_os = "macos")]
static HUD_CENTER: Mutex<Option<(f32, f32)>> = Mutex::new(None);

/// Whether the cleanup plate asked the HUD to give way. Never off macOS.
pub(super) fn asked() -> bool {
    GIVE_WAY.load(Ordering::Acquire)
}

/// The HUD records that its overlay went on or off screen.
pub(super) fn shown(shown: bool) {
    HUD_SHOWN.store(shown, Ordering::Release);
}

/// Keeps the HUD's overlay, so [`give_way`] can hide it.
#[cfg(target_os = "macos")]
pub(super) fn keep(overlay: &OverlayHandle<Hud>) {
    let _ = HUD.set(overlay.clone());
}

/// Where the cleanup plate takes the HUD's place: its plate's centre.
#[cfg(target_os = "macos")]
pub(crate) fn hud_center() -> Option<(f32, f32)> {
    *HUD_CENTER.lock().unwrap_or_else(|error| error.into_inner())
}

#[cfg(target_os = "macos")]
pub(super) fn set_hud_center(center: Option<(f32, f32)>) {
    *HUD_CENTER.lock().unwrap_or_else(|error| error.into_inner()) = center;
}

/// Whether the HUD is on screen, so the cleanup plate makes room below it.
#[cfg(target_os = "macos")]
pub(crate) fn hud_shown() -> bool {
    HUD_SHOWN.load(Ordering::Acquire)
}

/// Hides the HUD for the cleanup plate (`true`) until it is done (`false`);
/// a new capture shows the HUD again either way.
#[cfg(target_os = "macos")]
pub(crate) fn give_way(give: bool) {
    GIVE_WAY.store(give, Ordering::Release);
    if give && let Some(overlay) = HUD.get() {
        overlay.update(|hud, _| {
            if !hud.model.capturing() {
                hud.hide();
            }
        });
    }
}
