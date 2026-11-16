//! The setup machine's "New for you": the list of features that fit,
//! written by the app with no question asked.

use super::{Effect, Env, Flow};
use crate::assistant::setup::fresh;
use crate::assistant::setup::since::Since;
use crate::assistant::store::Turn;
use crate::desktop::i18n::tr;

impl Flow {
    /// "New for you": one turn with the list to tick, written by the app
    /// from the answers a setup would start from now. With nothing to
    /// tick, the look itself is kept, so what came isn't counted again.
    pub(super) fn fresh(&mut self, since: Option<Since>, env: &Env, out: &mut Vec<Effect>) {
        let context = self.context(env);
        let answers = fresh::answers(since.as_ref(), &context.settings, &context.state);
        let (releases, usage, last_ms) = match &since {
            Some(since) => (
                since.releases.as_slice(),
                since.usage.as_ref(),
                since.last.as_ref().map_or(0, |last| last.at_ms),
            ),
            None => (&[][..], None, 0),
        };
        let found = fresh::fresh(&answers, &context.settings, &context.state, usage, releases);
        let card = fresh::card(&found, &context.settings, &context.state);
        let settled = card.as_ref().is_none_or(|card| card.items.is_empty());
        let turn = Turn {
            question: tr(fresh::QUESTION).to_owned(),
            answer: fresh::opening(&found, last_ms),
            answers,
            cards: card.into_iter().collect(),
            created_ms: env.now_ms,
            ..Turn::default()
        };
        self.push(turn, out);
        if settled {
            self.keep(None, env, out);
        }
    }
}
