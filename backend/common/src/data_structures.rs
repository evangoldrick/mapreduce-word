use std::collections::HashMap;

use std::sync::{Arc, RwLock};
pub type JobIdType = u32;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JobJson {
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JsonString {
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct StandardErrorString {
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Jsonu32 {
    pub number: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JobHashMap {
    pub jobs: HashMap<JobIdType, ControllerJob>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JsonInt {
    pub job_id: JobIdType,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ControllerJob {
    pub text: String,
}

impl ControllerJob {
    pub fn from(j: &JobJson) -> Self {
        ControllerJob {
            text: j.text.clone(),
        }
    }
}

pub struct MainStateData {
    pub new_jobs: Arc<RwLock<HashMap<JobIdType, JobJson>>>,
    pub in_progress_jobs: Arc<RwLock<HashMap<JobIdType, ControllerJob>>>,
    pub server_status: Arc<RwLock<ServerState>>,
}

pub struct MainState {
    pub state: Arc<RwLock<MainStateData>>,
}

pub enum ServerState {
    Running,
    StopRequested,
    Stopping,
    Stopped,
}
