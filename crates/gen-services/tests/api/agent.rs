use gen_services::services::AgentService;
use gen_types::Agent;

use crate::context::TestContext;
#[test]
fn adding_agent_succeeds() {
    let ctx = TestContext::default();

    let agent = Agent::default();
    let agent_id = agent.id();
    ctx.service.add_agent_raw("user", agent).unwrap();

    let agent = ctx.agent_repo.get_agent(&agent_id).unwrap().unwrap();
    assert_eq!(agent.updated_by(), "user");
}
