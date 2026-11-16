//! What the start page's rules read (`assistant::suggest::Signals`), filled
//! from what the window already has: History's newest dictations, the
//! meetings (the Meetings pane's list, else a scan of the folder on a
//! thread of its own), the conversations and setups kept, the update, and
//! the Labs features new since the last setup. A scene's come from its
//! fixture; nothing here reads the disk in one.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::time::{Duration, Instant};

use super::super::super::Ui;
use super::super::super::meetings::Status;
use crate::assistant::setup::flow::{self, Step};
use crate::assistant::setup::{self, follow, since};
use crate::assistant::store::Store;
use crate::assistant::suggest::{From, Left, Met, Signals};
use crate::meeting::query::Library;

/// How many of History's newest entries a run of failures is read from.
const DICTATIONS: usize = 30;
/// The first look for what stopped working, after launch.
const FIRST_LOOK: Duration = Duration::from_secs(10);
/// How often it looks again while nothing else asks it to.
const LOOK_EVERY: Duration = Duration::from_secs(5 * 60);
/// How often History's revision is checked for new dictations.
const REVISION_EVERY: Duration = Duration::from_secs(10);
/// A scan this fresh serves a pane opened again.
const FRESH_SCAN: Duration = Duration::from_secs(30);

/// The meetings read off the UI thread, and when to read them again.
#[derive(Default)]
pub(super) struct Watch {
    /// When the window first looked: the first scan waits [`FIRST_LOOK`].
    since: Option<Instant>,
    /// When the last scan started.
    scanned_at: Option<Instant>,
    scanning: Option<Receiver<Vec<Met>>>,
    /// The finished meetings the last scan found, newest first.
    pub(super) meetings: Option<Vec<Met>>,
    revision: Option<u64>,
    revision_at: Option<Instant>,
    /// Kept between scans: a meeting read before and unchanged since is
    /// not read again.
    library: Option<Arc<Library>>,
}

/// What changed while watching.
#[derive(Default)]
pub(super) struct Looked {
    /// A scan landed or History has new entries: read the mark again.
    pub(super) again: bool,
}

impl Watch {
    /// Takes a finished scan, starts one when due, and checks History's
    /// revision now and then. Never reads a meeting on this thread.
    pub(super) fn look(
        &mut self,
        ui_revision: impl FnOnce() -> Option<u64>,
        now_ms: u64,
    ) -> Looked {
        let now = Instant::now();
        let mut looked = Looked::default();
        if let Some(scanning) = &self.scanning {
            match scanning.try_recv() {
                Ok(meetings) => {
                    self.meetings = Some(meetings);
                    self.scanning = None;
                    looked.again = true;
                }
                Err(TryRecvError::Disconnected) => self.scanning = None,
                Err(TryRecvError::Empty) => {}
            }
        }
        let since = *self.since.get_or_insert(now);
        let due = match self.scanned_at {
            None => now.duration_since(since) >= FIRST_LOOK,
            Some(at) => now.duration_since(at) >= LOOK_EVERY,
        };
        if due {
            self.scan(now_ms);
        }
        if self
            .revision_at
            .is_none_or(|at| now.duration_since(at) >= REVISION_EVERY)
        {
            self.revision_at = Some(now);
            let revision = ui_revision();
            if revision != self.revision {
                self.revision = revision;
                looked.again |= self.meetings.is_some();
            }
        }
        looked
    }

    /// Starts a scan unless one runs or a fresh one landed.
    pub(super) fn refresh(&mut self, now_ms: u64) {
        if self.scanned_at.is_none_or(|at| at.elapsed() >= FRESH_SCAN) {
            self.scan(now_ms);
        }
    }

    fn scan(&mut self, now_ms: u64) {
        if self.scanning.is_some() {
            return;
        }
        self.scanned_at = Some(Instant::now());
        if self.library.is_none() {
            let Ok(root) = crate::meeting::root() else {
                return;
            };
            self.library = Some(Arc::new(Library::new(root)));
        }
        let Some(library) = self.library.clone() else {
            return;
        };
        let (send, receive) = channel();
        let spawned = std::thread::Builder::new()
            .name("assistant-suggestions".into())
            .spawn(move || {
                let _ = send.send(scan(&library, now_ms));
            });
        if spawned.is_ok() {
            self.scanning = Some(receive);
        }
    }
}

/// The finished meetings on disk, newest first; none when the folder
/// can't be read.
fn scan(library: &Library, now_ms: u64) -> Vec<Met> {
    let Ok(scan) = library.scan() else {
        return Vec::new();
    };
    let mut meetings: Vec<Met> = scan
        .meetings
        .iter()
        .filter(|meeting| {
            meeting.status == crate::meeting::MeetingStatus::Complete
                && !meeting.in_progress
                && !crate::meeting::notes_in_progress(&meeting.id)
        })
        .map(|meeting| Met {
            id: meeting.id.clone(),
            title: meeting.title(),
            start_ms: meeting.start_ms(),
            duration_ms: meeting.duration_ms(now_ms),
            notes_failed: meeting.manifest.notes_error.is_some(),
        })
        .collect();
    meetings.sort_by_key(|met| std::cmp::Reverse(met.start_ms));
    meetings
}

/// Whether Clean up with AI is on for dictation, as Output's switch says.
pub(super) fn cleanup_on(ui: &Ui) -> bool {
    !matches!(
        crate::assistant::facts::cleanup(&ui.settings),
        crate::assistant::facts::Cleanup::Off
    )
}

/// The finished meetings: the Meetings pane's while it holds them, else
/// the last scan's.
fn meetings(ui: &Ui) -> Vec<Met> {
    if ui.meetings.meetings.is_empty() {
        return (ui.assistant.suggestions.watch.meetings.clone()).unwrap_or_default();
    }
    let mut meetings: Vec<Met> = ui
        .meetings
        .meetings
        .iter()
        .filter(|meeting| meeting.status == Status::Complete && !meeting.notes_running)
        .map(|meeting| Met {
            id: meeting.id.clone(),
            title: meeting.title.clone(),
            start_ms: meeting.start_ms,
            duration_ms: meeting.duration_ms,
            notes_failed: meeting.notes_error.is_some(),
        })
        .collect();
    meetings.sort_by_key(|met| std::cmp::Reverse(met.start_ms));
    meetings
}

/// What the sidebar's mark reads: the dictations and the meetings only.
pub(super) fn for_mark(ui: &Ui) -> Signals {
    Signals {
        now_ms: ui.assistant.now_ms(),
        cleanup_on: cleanup_on(ui),
        cleaned: ui.history.cleanups(DICTATIONS),
        meetings: meetings(ui),
        ..Signals::default()
    }
}

/// Everything the start page's rules read. A scene's own signals, else,
/// in a scene, only what the window holds in memory.
pub(super) fn gather(ui: &mut Ui) -> Signals {
    let pane = &ui.assistant;
    let state = &pane.suggestions;
    if let Some(scene) = &state.scene {
        return scene.clone();
    }
    let fixture = ui.is_fixture() || pane.fixture;
    let from = state.from;
    let mut signals = if fixture {
        Signals {
            now_ms: pane.now_ms(),
            cleanup_on: cleanup_on(ui),
            ..Signals::default()
        }
    } else {
        for_mark(ui)
    };
    signals.from = from;
    signals.open_meeting = state
        .open_meeting
        .clone()
        .filter(|_| from == Some(From::Meetings));
    signals.left = left(&pane.store, pane.open);
    signals.updated = super::super::whats_new::offered(ui).map(str::to_owned);
    if !fixture {
        signals.new_labs = new_labs(ui);
    }
    signals
}

/// The newest setup begun in another conversation and left before its
/// plan was decided: rounds still to answer, or the plan waiting.
pub(super) fn left(store: &Store, open: Option<u32>) -> Option<Left> {
    store
        .conversations()
        .iter()
        .filter(|conversation| Some(conversation.id) != open)
        .filter_map(|conversation| {
            let plan = match flow::step(&conversation.turns, store.setups()) {
                Step::Plan { .. } => true,
                Step::Basics { .. }
                | Step::Follow { .. }
                | Step::Answered { .. }
                | Step::Stalled { .. } => false,
                _ => return None,
            };
            let started_ms = store
                .setups()
                .iter()
                .find(|setup| setup.conversation == conversation.id && setup.started_ms != 0)
                .map(|setup| setup.started_ms)
                .or_else(|| conversation.turns.first().map(|turn| turn.created_ms))?;
            Some(Left {
                conversation: conversation.id,
                started_ms,
                plan,
            })
        })
        .max_by_key(|left| left.started_ms)
}

/// The Labs features new since the last setup that fit and are off, read
/// again only when the settings or the setups changed: the follow-ups'
/// rules are not cheap.
fn new_labs(ui: &mut Ui) -> Vec<(String, String)> {
    let setups = ui.assistant.store.setups().len();
    if let Some(read) = &ui.assistant.suggestions.labs
        && read.settings == ui.settings
        && read.setups == setups
    {
        return read.labs.clone();
    }
    let labs = fitting(ui);
    ui.assistant.suggestions.labs = Some(NewLabs {
        settings: ui.settings.clone(),
        setups,
        labs: labs.clone(),
    });
    labs
}

/// The Labs features new for the person, and the settings and number of
/// setups they were read at.
pub(super) struct NewLabs {
    settings: crate::app_settings::AppSettings,
    setups: usize,
    labs: Vec<(String, String)>,
}

/// The Labs features releases brought after the last setup (or, without
/// one, after the first version this install ran) that setup's follow-ups
/// would offer now: they fit the answers, are off, and can be used here.
/// Without either date nothing counts as new.
fn fitting(ui: &Ui) -> Vec<(String, String)> {
    let last = since::last(ui.assistant.store.setups()).cloned();
    let after = match &last {
        Some(last) => last.at_ms,
        None => crate::assistant::releases::history::read()
            .first()
            .map_or(0, |first| first.first_ms),
    };
    if after == 0 {
        return Vec::new();
    }
    let keys: Vec<String> = since::releases_after(after)
        .into_iter()
        .flat_map(|release| release.keys)
        .filter(|key| key.starts_with("labs."))
        .collect();
    if keys.is_empty() {
        return Vec::new();
    }
    let mut context = super::super::changes::context(ui);
    let kept = ui.assistant.store.setups();
    context.state.answered = crate::assistant::setup::past::answered_before(
        kept,
        &context.settings,
        since::changed_since,
    );
    context.state.declined = crate::assistant::setup::past::declined(kept);
    let answers = match last {
        Some(last) => last.answers,
        None => {
            let spoken = setup::Spoken::of(&context.settings, &context.state);
            let mut asks = setup::questions_for(context.state.os, spoken);
            setup::preselect(&mut asks, &context.settings, &context.state);
            for ask in &mut asks {
                ask.done = true;
            }
            setup::answers(&asks)
        }
    };
    follow::candidates(&answers, &context.settings, &context.state)
        .into_iter()
        .filter(|ask| keys.contains(&ask.id))
        .map(|ask| (ask.id, ask.hint))
        .collect()
}
