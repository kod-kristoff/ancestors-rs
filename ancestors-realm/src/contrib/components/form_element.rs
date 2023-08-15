use tui_realm_stdlib::props::{INPUT_PLACEHOLDER, INPUT_PLACEHOLDER_STYLE};
use tui_realm_stdlib::{Input, Label};
use tuirealm::props::Layout;
use tuirealm::tui::layout::Constraint;
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, Borders, Color, Style},
    AttrValue, Attribute, MockComponent, Props,
};

#[derive(Default)]
pub struct FormElement {
    props: Props,
    label: Label,
    input: Input,
}

impl FormElement {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.label.attr(Attribute::Foreground, AttrValue::Color(fg));
        self.input.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn default_layout(mut self) -> Self {
        self.attr(
            Attribute::Layout,
            AttrValue::Layout(
                Layout::default()
                    .direction(tuirealm::tui::layout::Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()),
                // .constraints([Constraint::Min(40), Constraint::Min(50)].as_ref()),
            ),
        );
        self
    }

    pub fn label<S: AsRef<str>>(mut self, text: S) -> Self {
        self.label = self.label.text(text);
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.attr(Attribute::Layout, AttrValue::Layout(layout));
        self
    }

    pub fn placeholder<S: AsRef<str>>(mut self, placeholder: S, style: Style) -> Self {
        self.input.attr(
            Attribute::Custom(INPUT_PLACEHOLDER),
            AttrValue::String(placeholder.as_ref().to_string()),
        );
        self.input.attr(
            Attribute::Custom(INPUT_PLACEHOLDER_STYLE),
            AttrValue::Style(style),
        );
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }
}

impl MockComponent for FormElement {
    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.input.perform(cmd)
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props
            .get(attr)
            .or_else(|| self.input.query(attr))
            .or_else(|| self.label.query(attr))
    }

    fn state(&self) -> tuirealm::State {
        self.input.state()
    }

    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        // Make a span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make block
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let div = tui_realm_stdlib::utils::get_block(borders, title, true, None);
            // Render block
            frame.render_widget(div, area);
            // Render children
            if let Some(layout) = self.props.get(Attribute::Layout).map(|x| x.unwrap_layout()) {
                log::trace!("drawing layout in area {:?}", area);
                // make chunks
                let chunks = layout.chunks(area);
                log::trace!("area split in {} chunks", chunks.len());
                // iter chunks
                log::trace!("drawing label in area {:?}", chunks[0]);
                self.label.view(frame, chunks[0]);
                log::trace!("drawing input in area {:?}", chunks[1]);
                self.input.view(frame, chunks[1]);
                // for (i, chunk) in chunks.into_iter().enumerate() {
                //     if let Some(child) = self.children.get_mut(i) {
                //         child.view(render, chunk);
                //     }
                // }
            }
        }
    }
}
