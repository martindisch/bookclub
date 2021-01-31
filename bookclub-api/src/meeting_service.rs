use std::{error, fmt};

use crate::{
    meeting_repository::{Error as RepositoryError, MeetingRepository},
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
        Ok(self.repository.meetings().await?)
    }
}

/// The error type for everything.
#[derive(Debug)]
pub enum Error {
    Internal(RepositoryError),
    User,
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        Self::Internal(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal(_) => write!(f, "An internal error occurred"),
            Self::User => write!(f, "User error (TBD)"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Internal(ref e) => Some(e),
            Self::User => None,
        }
    }
}
