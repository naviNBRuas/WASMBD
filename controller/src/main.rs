mod models;

use axum::{extract::State, routing::get, Json, Router};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use std::net::SocketAddr;

use models::{Agent, AgentStatus, Task, TaskStatus};

#[derive(Default)]
struct AppState {
    agents: HashMap<String, Agent>,
    tasks: HashMap<String, Task>,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app_state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/agent/register", get(register_agent))
        .route("/agent/status", get(get_agent_status))
        .route("/task/submit", get(submit_task))
        .with_state(app_state);

    // run it with hyper on `localhost:3000`
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root_handler() -> String {
    "Hello, Controller!".to_string()
}

async fn register_agent(State(state): State<Arc<Mutex<AppState>>>) -> Json<String> {
    let mut app_state = state.lock().await;
    let agent_id = Uuid::new_v4().to_string();
    let agent = Agent {
        id: agent_id.clone(),
        last_seen: std::time::Instant::now(),
        status: AgentStatus::Online,
    };
    app_state.agents.insert(agent_id.clone(), agent);
    Json(format!("Agent registered with ID: {}", agent_id))
}

async fn get_agent_status(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, Agent>> {
    let app_state = state.lock().await;
    Json(app_state.agents.clone())
}

async fn submit_task(State(state): State<Arc<Mutex<AppState>>>) -> Json<String> {
    let mut app_state = state.lock().await;
    let task_id = Uuid::new_v4().to_string();
    let task = Task {
        id: task_id.clone(),
        agent_id: "".to_string(), // To be assigned later
        module_name: "hello-world".to_string(), // Example
        payload: "{}".to_string(), // Example
        status: TaskStatus::Pending,
    };
    app_state.tasks.insert(task_id.clone(), task);
    Json(format!("Task submitted with ID: {}", task_id))
}