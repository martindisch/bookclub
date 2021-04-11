use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::meeting_service::MeetingService;

pub mod handlers;
pub mod meeting_repository;
pub mod meeting_service;

/// A meeting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub id: String,
    pub date: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub pitched_by: String,
    pub first_suggested: DateTime<Utc>,
    pub supporters: Vec<String>,
}

/// A request for creating a new meeting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeeting {
    pub date: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub pitched_by: String,
    pub first_suggested: DateTime<Utc>,
    pub supporters: Vec<String>,
}

/// A request for updating a meeting.
#[derive(Debug)]
pub struct UpdateMeeting {
    pub id: String,
    pub date: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub pitched_by: Option<String>,
    pub first_suggested: Option<DateTime<Utc>>,
    pub supporters: Option<Vec<String>>,
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
