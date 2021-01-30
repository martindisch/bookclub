use crate::{CreateMeeting, Meeting};

use futures::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};

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
    pub async fn insert_meeting(
        &self,
        create_meeting: &CreateMeeting,
    ) -> String {
        let document = bson::to_document(create_meeting).unwrap();
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
            meetings.push(
                bson::from_document::<MeetingWithOid>(document)
                    .unwrap()
                    .into(),
            );
        }

        meetings
    }
}

/// A meeting with its ID as it is stored in MongoDB.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingWithOid {
    #[serde(rename(deserialize = "_id"))]
    pub id: ObjectId,
    pub date: Option<String>,
    pub location: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub pitched_by: String,
    pub first_suggested: String,
    pub supporters: Vec<String>,
}

impl Into<Meeting> for MeetingWithOid {
    fn into(self) -> Meeting {
        Meeting {
            id: self.id.to_hex(),
            date: self.date,
            location: self.location,
            title: self.title,
            author: self.author,
            description: self.description,
            pitched_by: self.pitched_by,
            first_suggested: self.first_suggested,
            supporters: self.supporters,
        }
    }
}
