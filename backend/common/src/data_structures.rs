#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TextJson {
    pub text: String,
    pub job_id: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JustInt {
    pub job_id: u32,
}

pub struct MainState {
    pub job_map: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<TextJson>>>,
    pub server_status: std::sync::Arc<std::sync::Mutex<ServerState>>,
}

pub enum ServerState {
    Running,
    StopRequested,
    Stopping,
    Stopped,
}
