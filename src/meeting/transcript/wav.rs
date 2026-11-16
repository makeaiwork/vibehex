//! Labs › Files: an audio file's 16 kHz WAV transcribed in meeting chunks.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use color_eyre::eyre::{Result, eyre};

use super::super::echo::EchoEnvelope;
use super::super::{MeetingSource, TranscriptEntry};
use super::{ChunkModel, TrackTranscript, samples_ms, transcribe_file};
use crate::transcription::models::TranscriptionSelection;

/// Labs › Files: transcribes the 16 kHz mono WAV at `path` as one track in
/// meeting chunks, yielding to dictation like a meeting. `selection` is a
/// model of its own; `None` shares dictation's. `progress` hears how many
/// milliseconds are done; `cancelled` ends it between chunks.
pub(crate) fn transcribe_wav(
    path: &Path,
    selection: Option<TranscriptionSelection>,
    vocabulary: bool,
    cancelled: &AtomicBool,
    progress: &mut dyn FnMut(u64),
) -> Result<Vec<TranscriptEntry>> {
    let fixed = selection.is_some();
    let selection = selection.unwrap_or_else(|| crate::app_settings::transcription_selection().1);
    let mut model = ChunkModel {
        fixed,
        vocabulary,
        ..ChunkModel::new(selection)
    };
    let transcript = transcribe_file(
        |chunk| {
            if cancelled.load(Ordering::Acquire) {
                return Err(eyre!("cancelled"));
            }
            let transcript = model.transcribe_blocking(&chunk.samples)?;
            progress(samples_ms(chunk.start + chunk.samples.len()));
            Ok(transcript)
        },
        path,
        TrackTranscript::new(MeetingSource::System, 0),
        &mut EchoEnvelope::default(),
    )?;
    Ok(transcript.entries)
}
