use crate::Meeting;

use futures::StreamExt;
use mongodb::{bson, Collection};

/// Gives access to the MongoDB collection for meetings.
pub struct MeetingRepository {
    meetings: Collection,
}

impl MeetingRepository {
    /// Creates a new repository.
    pub fn new(meetings: Collection) -> Self {
        Self { meetings }
    }

    /// Inserts a new meeting, returning the ID.
    pub async fn insert_meeting(&self, meeting: &Meeting) -> String {
        let document = bson::to_document(meeting).unwrap();
        let insert_one_result =
            self.meetings.insert_one(document, None).await.unwrap();
        let id = insert_one_result.inserted_id.as_object_id().unwrap();

        id.to_hex()
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
