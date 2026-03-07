use gen_services::services::DocumentService;
use gen_types::Document;

use crate::context::TestContext;
#[test]
fn adding_document_succeeds() -> eyre::Result<()> {
    let ctx = TestContext::new()?;

    let document = Document::default();
    let document_id = document.id();
    ctx.service.add_document_raw("user", document)?;

    let document = ctx.document_repo.get_document(&document_id)?.unwrap();
    assert_eq!(document.updated_by(), "user");
    Ok(())
}
