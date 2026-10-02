use std::sync::atomic::{AtomicU64, Ordering};

use freya::prelude::Element;

static TAB_IDS: AtomicU64 = AtomicU64::new(0);
static PANEL_IDS: AtomicU64 = AtomicU64::new(0);

pub trait PanelTab {
    // fn on_close(&mut self, _app_state: &mut AppState) {}

    // fn on_settings_changed(&mut self, _app_settings: &AppSettings) {}

    // fn get_data(&self) -> PanelTabData;

    fn render(&self) -> fn(&TabProps) -> Element;

    // fn as_any(&self) -> &dyn Any;

    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, PartialEq)]
pub struct TabProps {
    pub tab_id: TabId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PanelId(u64);

impl PanelId {
    pub fn new() -> Self {
        let n = PANEL_IDS.fetch_add(1, Ordering::Relaxed);
        Self(n)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TabId(u64);

impl TabId {
    pub fn new() -> Self {
        let n = TAB_IDS.fetch_add(1, Ordering::Relaxed);
        Self(n)
    }
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
