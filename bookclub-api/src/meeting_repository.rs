use crate::Meeting;

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
}
