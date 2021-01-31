use serde::{Deserialize, Serialize};

pub mod handlers;
mod meeting_repository;
mod meeting_service;

pub use meeting_repository::MeetingRepository;
pub use meeting_service::{Error, MeetingService};

/// A meeting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub id: String,
    pub date: Option<String>,
    pub location: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub pitched_by: String,
    pub first_suggested: String,
    pub supporters: Vec<String>,
}

/// A request for creating a new meeting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeeting {
    pub date: Option<String>,
    pub location: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub pitched_by: String,
    pub first_suggested: String,
    pub supporters: Vec<String>,
}

/// Poor man's DI container.
pub struct ServiceContainer {
    meeting_service: MeetingService,
}

impl ServiceContainer {
    /// Creates a new container.
    pub fn new(meeting_service: MeetingService) -> Self {
        Self { meeting_service }
    }
}
