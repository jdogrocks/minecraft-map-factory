use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Unique identifier for a generation job.
pub type JobId = usize;

/// State of a generation job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Queued,
    Running,
    Completed,
    Failed,
    Retrying,
}

/// A map generation job.
#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub location_idx: usize,
    pub state: JobState,
    pub attempt: u32,
    pub bbox_override: Option<[f64; 4]>,
}

/// Result of a completed generation job.
#[derive(Debug)]
pub struct JobResult {
    pub job_id: JobId,
    pub location_idx: usize,
    pub success: bool,
    pub duration: Duration,
    pub output_path: Option<PathBuf>,
    pub error: Option<String>,
}

impl Job {
    pub fn new(id: JobId, location_idx: usize) -> Self {
        Self {
            id,
            location_idx,
            state: JobState::Queued,
            attempt: 0,
            bbox_override: None,
        }
    }
}
