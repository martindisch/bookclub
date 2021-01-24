use crate::Meeting;

use futures::StreamExt;
use mongodb::Collection;

/// Gives access to the MongoDB collection for meetings.
pub struct MeetingRepository {
    meetings: Collection,
}

impl MeetingRepository {
    /// Creates a new repository.
    pub fn new(meetings: Collection) -> Self {
        Self { meetings }
    }

    /// Inserts a new meeting.
    pub async fn insert_meeting(&self, meeting: &Meeting) {
        let document = bson::to_document(meeting).unwrap();
        self.meetings.insert_one(document, None).await.unwrap();
    }

    /// Returns all meetings.
    pub async fn meetings(&self) -> Vec<Meeting> {
        let mut cursor = self.meetings.find(None, None).await.unwrap();
        let mut meetings = Vec::new();

        while let Some(Ok(document)) = cursor.next().await {
            meetings.push(bson::from_document(document).unwrap());
        }

        meetings
    }
}
