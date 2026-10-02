#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EditorView {
    #[default]
    Panels,
}

impl EditorView {
    pub fn is_popup(&self) -> bool {
        false
    }
}

impl std::fmt::Display for EditorView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Panels => f.write_str("Panels"),
        }
    }
}
