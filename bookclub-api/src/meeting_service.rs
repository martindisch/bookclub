use crate::{meeting_repository::MeetingRepository, Meeting};

/// Represents the meetings domain.
pub struct MeetingService {
    repository: MeetingRepository,
}

impl MeetingService {
    /// Creates a new service.
    pub fn new(repository: MeetingRepository) -> Self {
        Self { repository }
    }

    /// Creates a new meeting.
    pub async fn create_meeting(&self, meeting: &Meeting) {
        self.repository.insert_meeting(meeting).await;
    }
}
