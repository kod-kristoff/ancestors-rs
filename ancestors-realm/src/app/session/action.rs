/// Defines the action performed by the player in a turn
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    AddPerson,
    SaveSession,
}
