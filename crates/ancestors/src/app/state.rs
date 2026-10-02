use std::collections::HashMap;

use freya::{
    components::{DockNode, DockPanel, DockingModel, DropTarget}, radio::RadioChannel,
};

use crate::app::{
    panels_tabs::{PanelId, PanelTab, TabId},
    views::EditorView,
};

pub struct AppState {
    // Views
    pub focused_view: EditorView,
    pub previous_focused_view: Option<EditorView>,

    // Panels
    pub focused_panel: Option<PanelId>,
    pub panel_tree: Option<DockNode<TabId, PanelId>>,

    // Tabs
    pub tabs: HashMap<TabId, Box<dyn PanelTab>>,
}

impl AppState {
    pub fn new() -> Self {
        let panel_id = PanelId::new();
        Self {
            // Views
            focused_view: EditorView::default(),
            previous_focused_view: None,
            // Panels
            focused_panel: Some(panel_id),
            panel_tree: Some(DockNode::Panel(DockPanel::new(panel_id, vec![]))),
            // Tabs
            tabs: HashMap::new(),
        }
    }

    pub fn tab(&self, tab_id: &TabId) -> &(dyn PanelTab + 'static) {
        self.tabs.get(tab_id).unwrap().as_ref()
    }

    fn move_tab(&mut self, tab: TabId, target: DropTarget<PanelId>) -> bool {
        // Panel the tab currently lives in
        let source_panel = self.panel_tree.as_ref().and_then(|tree| tree.find_tab(&tab)).map(|(panel_id, _)| panel_id);

        let Some(tree) = self.panel_tree.as_mut() else {
            return false;
        }
    }
}

impl DockingModel for AppState {
    type TabId = TabId;
    type PanelId = PanelId;
    type DropValue = DropValue;

    fn root(&self) -> Option<&freya::prelude::DockNode<Self::TabId, Self::PanelId>> {
        self.panel_tree.as_ref()
    }

    fn on_drop(
        &mut self,
        value: Self::DropValue,
        target: freya::prelude::DropTarget<Self::PanelId>,
    ) -> bool {
        match value {
            DropValue::Tab(tab) => self.move_tab(tab, target),
            DropValue::File(path) => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum AppChannel {
    /// Affects global components
    Global,
    /// Only affects the active tab
    ActiveTab,
}

impl RadioChannel<AppState> for AppChannel {}

/// What a drag can carry onto the docking area: an existing tab, or a file path
/// (dragged in from the file explorer) to open as a new tab.
#[derive(Clone, PartialEq)]
pub enum DropValue {
    Tab(TabId),
    File(std::path::PathBuf),
}

impl From<TabId> for DropValue {
    fn from(tab_id: TabId) -> Self {
        DropValue::Tab(tab_id)
    }
}
