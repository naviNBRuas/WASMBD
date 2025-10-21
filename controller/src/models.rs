use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    #[serde(with = "serde_millis")]
    pub last_seen: std::time::Instant,
    pub status: AgentStatus,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Online,
    Offline,
    Busy,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub agent_id: String,
    pub module_name: String,
    pub payload: String,
    pub status: TaskStatus,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}