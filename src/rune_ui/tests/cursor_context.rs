//! VibeHex: Labs › Rewrite reads nearby text's card while on: the line that
//! names where the text goes with the cleanup engine chosen now, and Send it
//! to cloud engines, which a cloud engine needs before it gets the text.

use super::*;
use crate::app_settings::CleanupRunsOn;

fn shows(rt: &Runtime<Ui>, line: &str) -> bool {
    texts(rt).iter().any(|text| text == line)
}

#[test]
fn the_nearby_text_card_names_where_it_goes_and_asks_for_the_cloud() {
    let mut rt = runtime("mac-production", true, (980.0, 2000.0));
    {
        let settings = &mut rt.state.settings;
        settings.labs.ai_cleanup_server = true;
        settings.dictation_processing.server.runs_on = CleanupRunsOn::Local;
        settings
            .dictation_processing
            .default_mode
            .post_processing
            .enabled = true;
    }
    open_labs(&mut rt);
    assert!(
        rt.find("cursor-context-destination").is_none(),
        "the line shows only while the lab is on"
    );
    tap(&mut rt, "lab-cursor-context");
    assert!(rt.state.settings.labs.cursor_context);
    assert!(shows(
        &rt,
        "With your cleanup engine now, the nearby text stays on this Mac."
    ));
    assert!(rt.find("cursor-context-cloud-switch").is_some());

    rt.state.settings.dictation_processing.server.runs_on = CleanupRunsOn::Codex;
    rt.invalidate();
    settle(&mut rt);
    assert!(shows(
        &rt,
        "Not used now: your cleanup runs on Codex. Turn on Send it to cloud engines to use it there."
    ));
    tap(&mut rt, "cursor-context-cloud-switch");
    assert!(rt.state.settings.labs.cursor_context_cloud);
    assert!(shows(
        &rt,
        "With your cleanup engine now, the nearby text is sent to Codex with each dictation."
    ));

    rt.state
        .settings
        .dictation_processing
        .default_mode
        .post_processing
        .enabled = false;
    rt.invalidate();
    settle(&mut rt);
    assert!(shows(&rt, "Nothing reads it now: AI cleanup is off."));
}
