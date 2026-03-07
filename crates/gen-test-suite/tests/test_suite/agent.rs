use gen_services::services::AgentService;
use gen_types::Agent;

use crate::context::TestContext;
#[test]
fn adding_agent_succeeds() -> eyre::Result<()> {
    // Arrange
    let ctx = TestContext::new()?;

    let agent = Agent::default();
    let agent_id = agent.id();
    ctx.service.add_agent_raw("user", agent.clone()).unwrap();

    // Act
    let actual = ctx.agent_repo.get_agent(&agent_id).unwrap().unwrap();
    let actuals = ctx.agent_repo.get_all_agents().unwrap();
    // Assert
    assert_eq!(actual.updated_by(), "user");
    assert_eq!(actual, agent);
    assert_eq!(actuals, &[agent]);
    Ok(())
}
