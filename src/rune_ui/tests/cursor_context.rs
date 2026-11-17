//! VibeHex: Labs › Rewrite reads nearby text's card while on: the one line
//! that names where the text goes with the cleanup engine chosen now.

use super::*;
use crate::app_settings::CleanupRunsOn;

fn shows(rt: &Runtime<Ui>, line: &str) -> bool {
    texts(rt).iter().any(|text| text == line)
}

#[test]
fn the_nearby_text_card_names_where_it_goes() {
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

    rt.state.settings.dictation_processing.server.runs_on = CleanupRunsOn::Codex;
    rt.invalidate();
    settle(&mut rt);
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
