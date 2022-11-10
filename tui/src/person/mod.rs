#[derive(Debug, PartialEq)]
pub enum PersonCommand {
    Add(String),
}

impl PersonCommand {
    pub fn new(command: String) -> Self {
        Self::Add(command)
    }
}
