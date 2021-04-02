//! Contains data-access logic.

use futures::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId, Bson, DateTime},
    Collection,
};
use serde::{Deserialize, Serialize};

use std::{error, fmt};

use crate::{CreateMeeting, Meeting};

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
    ) -> Result<String, Error> {
        let mut document = bson::to_document(create_meeting)?;

        // Sadly we need to replace DateTime<Utc> with the DateTime wrapper,
        // because DateTime<Utc> is serialized to String, whereas we want the
        // native BSON datetime type in the DB
        document.insert(
            "firstSuggested",
            Bson::DateTime(create_meeting.first_suggested),
        );
        document.insert(
            "date",
            create_meeting
                .date
                .map(Bson::DateTime)
                .unwrap_or(Bson::Null),
        );

        let insert_one_result =
            self.meetings.insert_one(document, None).await?;
        let id = insert_one_result
            .inserted_id
            .as_object_id()
            .ok_or(Error::BadObjectId)?;

        Ok(id.to_hex())
    }

    /// Returns all meetings.
    pub async fn meetings(&self) -> Result<Vec<Meeting>, Error> {
        let mut cursor = self.meetings.find(None, None).await?;
        let mut meetings = Vec::new();

        while let Some(Ok(document)) = cursor.next().await {
            meetings.push(
                bson::from_document::<MeetingDocument>(document)?.into(),
            );
        }

        Ok(meetings)
    }
}

/// A meeting as it is stored in MongoDB.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MeetingDocument {
    #[serde(rename(deserialize = "_id"))]
    id: ObjectId,
    date: Option<DateTime>,
    location: Option<String>,
    title: String,
    author: String,
    description: String,
    pitched_by: String,
    first_suggested: DateTime,
    supporters: Vec<String>,
}

#[allow(clippy::from_over_into)]
impl Into<Meeting> for MeetingDocument {
    fn into(self) -> Meeting {
        Meeting {
            id: self.id.to_hex(),
            date: self.date.map(DateTime::into),
            location: self.location,
            title: self.title,
            author: self.author,
            description: self.description,
            pitched_by: self.pitched_by,
            first_suggested: self.first_suggested.into(),
            supporters: self.supporters,
        }
    }
}

/// The error type wrapping what can go wrong in the repository.
#[derive(Debug)]
pub enum Error {
    Serialization(bson::ser::Error),
    Deserialization(bson::de::Error),
    MongoDb(mongodb::error::Error),
    BadObjectId,
}

impl From<bson::ser::Error> for Error {
    fn from(err: bson::ser::Error) -> Self {
        Self::Serialization(err)
    }
}

impl From<bson::de::Error> for Error {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serialization(e) => e.fmt(f),
            Self::Deserialization(e) => e.fmt(f),
            Self::MongoDb(e) => e.fmt(f),
            Self::BadObjectId => write!(f, "Insert did not return ObjectId"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Serialization(ref e) => Some(e),
            Self::Deserialization(ref e) => Some(e),
            Self::MongoDb(ref e) => Some(e),
            Self::BadObjectId => None,
        }
    }
}
