use freya::prelude::Component;

use crate::app::panels_tabs::PanelId;

/// Content shown in a panel with no tabs
#[derive(Clone, PartialEq)]
pub struct EmptyPanel {
    pub panel_id: PanelId,
}

impl Component for EmptyPanel {
    fn render(&self) -> impl freya::prelude::IntoElement {
        let panel_id = self.panel_id;
    }
}
