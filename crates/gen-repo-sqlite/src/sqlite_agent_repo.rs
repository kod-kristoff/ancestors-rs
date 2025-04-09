use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use gen_services::repositories::{AgentRepository, SharedAgentRepository};
use gen_types::{Agent, AgentId};

use crate::models::{AgentInDb, NewAgent};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct SqliteAgentRepository {
    read_pool: DbPool,
    write_pool: DbPool,
}

impl SqliteAgentRepository {
    pub fn new(path: &str) -> Self {
        let manager = ConnectionManager::new(path);

        let read_pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("sqlite_repo: build read_pool");

        let manager = ConnectionManager::new(path);

        let write_pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("sqlite_repo: build write_pool");

        Self {
            read_pool,
            write_pool,
        }
    }

    pub fn arc_new(path: &str) -> SharedAgentRepository {
        Arc::new(Self::new(path))
    }
}

impl AgentRepository for SqliteAgentRepository {
    fn get_agent(
        &self,
        id: &AgentId,
    ) -> Result<Option<Agent>, gen_services::repositories::AgentRepositoryError> {
        use crate::schema::agents::dsl::agents;
        let mut conn = self.read_pool.get().unwrap();
        let agent = agents
            .find(id.db_id())
            .select(AgentInDb::as_select())
            .first(&mut conn)
            .optional();
        dbg!(&agent);
        match agent {
            Ok(None) => Ok(None),
            Ok(Some(agent)) => Ok(serde_json::from_str(&agent.body).unwrap()),
            Err(err) => {
                todo!("handle error : {:?}", err)
            }
        }
    }

    fn get_all_agents(
        &self,
    ) -> Result<Vec<Agent>, gen_services::repositories::AgentRepositoryError> {
        use crate::schema::agents::dsl::agents;
        let mut conn = self.read_pool.get().unwrap();
        let all_agents = agents
            .select(AgentInDb::as_select())
            .load(&mut conn)
            .unwrap();
        let mut result = Vec::new();
        for agent in all_agents {
            result.push(serde_json::from_str(&agent.body).unwrap());
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

        let mut conn = self.write_pool.get().unwrap();
        let agent = diesel::insert_into(agents::table)
            .values(&new_agent)
            .execute(&mut conn)
            .unwrap();
        dbg!(agent);
        Ok(())
    }
}
