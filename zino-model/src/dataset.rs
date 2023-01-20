use serde::{Deserialize, Serialize};
use zino_core::{database::Model, datetime::DateTime, request::Validation, Map, Uuid};
use zino_derive::Schema;

/// The dataset model.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct Dataset {
    // Basic fields.
    id: Uuid,
    #[schema(not_null, index = "text")]
    name: String,
    #[schema(default = "Dataset::model_namespace", index = "hash")]
    namespace: String,
    #[schema(default = "internal")]
    visibility: String,
    #[schema(default = "active", index = "hash")]
    status: String,
    #[schema(index = "text")]
    description: String,

    // Info fields.
    project_id: Uuid, // group.id, group.namespace = "*:project", group.subject = "user"
    task_id: Option<Uuid>, // task.id
    valid_from: DateTime,
    expires_at: DateTime,
    #[schema(index = "gin")]
    tags: Vec<Uuid>, // tag.id, tag.namespace = "*:dataset"

    // Extensions.
    content: Map,
    metrics: Map,
    extras: Map,

    // Revisions.
    manager_id: Uuid,    // user.id
    maintainer_id: Uuid, // user.id
    #[schema(index = "btree")]
    created_at: DateTime,
    #[schema(index = "btree")]
    updated_at: DateTime,
    version: u64,
    edition: u32,
}

impl Model for Dataset {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Self::default()
        }
    }

    fn read_map(&mut self, data: Map) -> Validation {
        let mut validation = Validation::new();
        if let Some(result) = Validation::parse_uuid(data.get("id")) {
            match result {
                Ok(id) => self.id = id,
                Err(err) => validation.record_fail("id", err),
            }
        }
        if let Some(name) = Validation::parse_string(data.get("name")) {
            self.name = name;
        }
        if self.name.is_empty() {
            validation.record_fail("name", "should be nonempty");
        }
        validation
    }
}
