//! Labs › Listen to the recording, once a meeting is transcribed: its
//! microphone is cleaned of the speakers' echo on a background thread (about
//! 20 s for 90 min), so its first Play loads in about a second.

use std::thread;

use super::{MeetingManifest, MeetingStatus};

/// Cleans a finished meeting's microphone for playback, when Listen to the
/// recording is on; a meeting that didn't complete is left alone, and tests
/// never read the saved settings for it.
pub(super) fn prepare(manifest: &MeetingManifest) {
    if cfg!(test) || manifest.status != MeetingStatus::Complete {
        return;
    }
    // Read from the saved settings, as the player's own switch is.
    if !crate::app_settings::AppSettings::saved_labs().playback {
        return;
    }
    let directory = match super::root() {
        Ok(root) => root.join(&manifest.id),
        Err(error) => {
            tracing::warn!("couldn't find the meetings folder to clean the microphone: {error:#}");
            return;
        }
    };
    let spawned = thread::Builder::new()
        .name("meeting-speaker-echo".into())
        .spawn(move || {
            crate::transcription::background_priority::deprioritize_background_thread();
            if let Err(error) = crate::files::playback::prepare_meeting(&directory) {
                tracing::warn!("couldn't clean the meeting's microphone: {error:#}");
            }
        });
    if let Err(error) = spawned {
        tracing::warn!("couldn't start cleaning the meeting's microphone: {error}");
    }
}
