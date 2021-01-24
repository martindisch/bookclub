use serde::{Deserialize, Serialize};

mod meeting_repository;
mod meeting_service;

pub use meeting_repository::MeetingRepository;
pub use meeting_service::MeetingService;

/// A meeting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    date: Option<String>,
    location: Option<String>,
    title: String,
    author: String,
    description: String,
    pitched_by: String,
    first_suggested: String,
    supporters: Vec<String>,
}
