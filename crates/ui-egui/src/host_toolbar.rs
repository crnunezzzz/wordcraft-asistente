//! Optional Android assistant state, integrated into the native ribbon tabs.
#[derive(Default)]
pub struct HostToolbar {
    pub action: Option<&'static str>,
    pub assistant_open: bool,
    pub busy: bool,
    pub status: String,
}

/// Help › Check for Updates: true when the host checks its own releases instead (a build with
/// the assistant must not be replaced by the original project's, which has none).
pub fn check_for_updates(app: &mut crate::WordApp) -> bool {
    match &mut app.services.host_toolbar {
        Some(host) => {
            host.action = Some("check_update");
            true
        }
        None => false,
    }
}

/// The ribbon tab that opens the host's assistant.
pub const TAB: &str = "Asistente";

/// Put the host's tab after Help (or last), when the host has one. The ribbon's only seams are
/// these three calls, so merging the original project rarely touches them.
pub fn add_tab(app: &crate::WordApp, tabs: &mut Vec<(String, bool)>) {
    if app.services.host_toolbar.is_some() {
        let index = tabs.iter().position(|(name, _)| name == "Help").map_or(tabs.len(), |i| i + 1);
        tabs.insert(index, (TAB.into(), true));
    }
}

/// Whether the host's tab shows as selected (`None`: `tab` is not the host's).
pub fn active(app: &crate::WordApp, tab: &str) -> Option<bool> {
    (tab == TAB).then(|| app.services.host_toolbar.as_ref().is_some_and(|h| h.assistant_open))
}

/// A click on `tab`: true when it was the host's tab (the host opens its panel).
pub fn clicked(app: &mut crate::WordApp, tab: &str) -> bool {
    if tab != TAB {
        return false;
    }
    if let Some(host) = &mut app.services.host_toolbar {
        host.action = Some("assistant");
    }
    true
}

#[cfg(test)]
mod tests {
    use egui_kittest::kittest::Queryable;

    #[test]
    fn assistant_is_after_help_and_keeps_the_editor_tab() {
        let services = crate::Services { host_toolbar: Some(Default::default()), ..Default::default() };
        let app = crate::WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Tablet")), services);
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1600.0, 900.0)).build_ui_state(
            |ui, app: &mut crate::WordApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(3);
        let help = h.get_by_label("Help").rect();
        let assistant = h.get_by_label("Asistente").rect();
        assert!(assistant.left() >= help.right());
        let tab = h.state().ui.tab.clone();
        h.get_by_label("Asistente").click();
        h.run_steps(2);
        assert_eq!(h.state().services.host_toolbar.as_ref().and_then(|h| h.action), Some("assistant"));
        assert_eq!(h.state().ui.tab, tab);
    }
}
