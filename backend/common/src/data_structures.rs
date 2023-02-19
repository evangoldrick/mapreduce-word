use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, LockResult, Mutex, RwLock};
pub type JobIdType = u32;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JobJson {
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JustInt {
    pub job_id: JobIdType,
}

#[derive(Debug)]
pub struct ControllerJob {
    pub text: String,
}

pub struct MainStateData {
    pub new_jobs: Arc<Mutex<HashMap<JobIdType, JobJson>>>,
    pub in_progress_jobs: Arc<Mutex<HashMap<JobIdType, ControllerJob>>>,
    pub server_status: Arc<Mutex<ServerState>>,
}
pub struct MainState {
    pub state: Arc<RwLock<MainStateData>>,
}

impl MainState {
    pub fn getState(&self) {
        match self.state.clone().read() {
            Ok(state) {
                state
            }
            Err(e) {
                eprint!("MainState was poisoned {}", e.to_string());
                eprint!("New MainState created");
                self.state = MainStateData {
                    in_progress_jobs: Arc::new(Mutex::new(HashMap::new())),
                    new_jobs: Arc::new(Mutex::new(HashMap::new())),
                    server_status: Arc::new(Mutex::new(ServerState::Unknown)),
                }
            }
        }
    }
}

pub enum ServerState {
    Running,
    StopRequested,
    Stopping,
    Stopped,
}
