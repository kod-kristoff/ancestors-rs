use ancestors::application::use_cases::EditPerson;

use crate::app::AppState;

pub fn edit_person(id: String, ctx: &mut AppState) -> eyre::Result<Option<EditPerson>> {
    Ok(None)
}
