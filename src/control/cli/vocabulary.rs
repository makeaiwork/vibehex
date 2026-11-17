//! `hex vocabulary`: list, add, remove and edit the saved terms.

use clap::Subcommand;

use super::{ControlCommand, Output, send};

#[derive(Subcommand, Debug)]
pub enum VocabularyCommand {
    /// Every term with its "sounds like" spellings.
    List {
        #[command(flatten)]
        output: Output,
    },
    /// Add a term, or more spellings to a term already in the list.
    Add {
        term: String,
        /// Spellings a transcript may show instead, comma-separated
        /// (`--sounds-like "capuchin,cap a chino"`).
        #[arg(long, value_name = "SPELLINGS")]
        sounds_like: Option<String>,
        #[command(flatten)]
        output: Output,
    },
    /// Remove a term and its spellings.
    Remove {
        term: String,
        #[command(flatten)]
        output: Output,
    },
    /// Rewrite a term's entry where it stands: rename it, and set exactly its
    /// spellings (`--sounds-like ""` clears them; left out, they stay).
    Edit {
        term: String,
        #[arg(long, value_name = "TERM")]
        rename: Option<String>,
        #[arg(long, value_name = "SPELLINGS")]
        sounds_like: Option<String>,
        #[command(flatten)]
        output: Output,
    },
}

pub(super) fn run(command: VocabularyCommand) -> u8 {
    match command {
        VocabularyCommand::List { output } => send(ControlCommand::VocabularyList, output),
        VocabularyCommand::Add {
            term,
            sounds_like,
            output,
        } => send(
            ControlCommand::VocabularyAdd {
                term,
                sounds_like: sounds_like.into_iter().collect(),
            },
            output,
        ),
        VocabularyCommand::Remove { term, output } => {
            send(ControlCommand::VocabularyRemove { term }, output)
        }
        VocabularyCommand::Edit {
            term,
            rename,
            sounds_like,
            output,
        } => send(
            ControlCommand::VocabularyEdit {
                term,
                rename,
                sounds_like: sounds_like.map(|spellings| vec![spellings]),
            },
            output,
        ),
    }
}
