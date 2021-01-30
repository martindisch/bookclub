use crate::{
    meeting_repository::{Error, MeetingRepository},
    CreateMeeting, Meeting,
};

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
    pub async fn create_meeting(
        &self,
        create_meeting: CreateMeeting,
    ) -> Result<Meeting, Error> {
        let id = self.repository.insert_meeting(&create_meeting).await?;

        Ok(Meeting {
            id,
            date: create_meeting.date,
            location: create_meeting.location,
            title: create_meeting.title,
            author: create_meeting.author,
            description: create_meeting.description,
            pitched_by: create_meeting.pitched_by,
            first_suggested: create_meeting.first_suggested,
            supporters: create_meeting.supporters,
        })
    }

    /// Returns all meetings.
    pub async fn meetings(&self) -> Result<Vec<Meeting>, Error> {
        self.repository.meetings().await
    }
}
