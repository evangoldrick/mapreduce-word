#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TextJson {
    pub text: String,
    pub job_id: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JustInt {
    pub job_id: u32,
}

pub enum ServerStates {
    Running,
    Stopping,
    Stopped,
}
