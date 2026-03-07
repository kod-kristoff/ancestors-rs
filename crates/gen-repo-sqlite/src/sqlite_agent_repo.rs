use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use eyre::Context;
use gen_services::repositories::{AgentRepository, AgentRepositoryError, SharedAgentRepository};
use gen_types::{Agent, AgentId};
use miette::IntoDiagnostic;

use crate::{
    models::{AgentInDb, NewAgent},
    pool::DbPool,
};

pub struct SqliteAgentRepository {
    db_pool: DbPool,
}

impl SqliteAgentRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn arc_new(db_pool: DbPool) -> SharedAgentRepository {
        Arc::new(Self::new(db_pool))
    }
}

impl AgentRepository for SqliteAgentRepository {
    fn get_agent(
        &self,
        id: &AgentId,
    ) -> Result<Option<Agent>, gen_services::repositories::AgentRepositoryError> {
        use crate::schema::agents::dsl::agents;
        let mut conn = self
            .db_pool
            .read()
            .wrap_err("failed to get read connection")?;
        let agent = agents
            .find(id.db_id())
            .select(AgentInDb::as_select())
            .first(&mut conn)
            .optional();
        dbg!(&agent);
        match agent {
            Ok(None) => Ok(None),
            Ok(Some(agent)) => Ok(serde_json::from_str(&agent.body).wrap_err_with(|| {
                format!("failed to deserialize agent body for id={}", &agent.id)
            })?),
            Err(err) => {
                todo!("handle error : {:?}", err)
            }
        }
    }

    fn get_all_agents(
        &self,
    ) -> Result<Vec<Agent>, gen_services::repositories::AgentRepositoryError> {
        use crate::schema::agents::dsl::agents;
        let mut conn = self
            .db_pool
            .read()
            .wrap_err("failed to get read connection")?;
        let all_agents = agents
            .select(AgentInDb::as_select())
            .load(&mut conn)
            .wrap_err("failed to load all agents")?;
        let mut result = Vec::new();
        for agent in all_agents {
            result.push(serde_json::from_str(&agent.body).wrap_err_with(|| {
                format!("failed to deserialize agent body for id={}", &agent.id)
            })?);
        }
        Ok(result)
    }
    fn save_agent(
        &self,
        agent: &Agent,
    ) -> Result<(), gen_services::repositories::AgentRepositoryError> {
        use crate::schema::agents;

        let id = agent.id().db_id();
        let body = serde_json::to_string(&agent).unwrap();
        let new_agent = NewAgent {
            id: &id,
            body: &body,
            updated: agent.updated().naive_utc(),
            updated_by: agent.updated_by(),
        };

        let mut conn = self
            .db_pool
            .write()
            .wrap_err("failed to get write connection")?;
        let agent = diesel::insert_into(agents::table)
            .values(&new_agent)
            .execute(&mut conn)
            .wrap_err_with(|| format!("failed to save agent with id={}", &agent.id()))?;
        dbg!(agent);
        Ok(())
    }
}
