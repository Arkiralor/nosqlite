use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "lowercase")]
pub enum MongoQuery {
    Find {
        find: String,
        filter: Option<Value>,
        sort: Option<Value>,
        limit: Option<u32>,
    },
    CreateCollection {
        name: String,
        options: Option<Value>,
    },
    DropCollection {
        name: String,
    },
    CreateUser {
        username: String,
        password: String,
        roles: Option<Value>,
    },
    DeleteUser {
        username: String,
    },
}

impl MongoQuery {
    pub fn parse(query: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(query)
    }
}
