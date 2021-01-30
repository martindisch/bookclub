use crate::{meeting_repository::MeetingRepository, Meeting, MeetingWithId};

/// Represents the meetings domain.
pub struct MeetingService {
    repository: MeetingRepository,
}

impl MeetingService {
    /// Creates a new service.
    pub fn new(repository: MeetingRepository) -> Self {
        Self { repository }
    }

    /// Creates a new meeting, returning the ID.
    pub async fn create_meeting(&self, meeting: Meeting) -> MeetingWithId {
        let id = self.repository.insert_meeting(&meeting).await;

        MeetingWithId {
            id: id.to_hex(),
            date: meeting.date,
            location: meeting.location,
            title: meeting.title,
            author: meeting.author,
            description: meeting.description,
            pitched_by: meeting.pitched_by,
            first_suggested: meeting.first_suggested,
            supporters: meeting.supporters,
        }
    }

    /// Returns all meetings.
    pub async fn meetings(&self) -> Vec<MeetingWithId> {
        self.repository.meetings().await
    }
}
