//! Contains data-access logic.

use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, DateTime, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};

use std::{error, fmt};

use crate::{CreateMeeting, Meeting, UpdateMeeting};

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

    /// Updates a meeting and returns the new one.
    pub async fn update_meeting(
        &self,
        update_meeting: UpdateMeeting,
    ) -> Result<Meeting, Error> {
        let updated_document = self
            .meetings
            .find_one_and_update(
                doc! {"_id": ObjectId::with_string(&update_meeting.id)?},
                build_update(update_meeting),
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(ReturnDocument::After))
                    .build(),
            )
            .await?
            .ok_or(Error::NoSuchMeeting)?;
        let updated_meeting: Meeting =
            bson::from_document::<MeetingDocument>(updated_document)?.into();

        Ok(updated_meeting)
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
    ObjectId(bson::oid::Error),
    MongoDb(mongodb::error::Error),
    BadObjectId,
    NoSuchMeeting,
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

impl From<bson::oid::Error> for Error {
    fn from(err: bson::oid::Error) -> Self {
        Self::ObjectId(err)
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
            Self::ObjectId(_) => write!(f, "Invalid ID."),
            Self::MongoDb(e) => e.fmt(f),
            Self::BadObjectId => write!(f, "Insert did not return ObjectId."),
            Self::NoSuchMeeting => write!(f, "Meeting does not exist."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Serialization(ref e) => Some(e),
            Self::Deserialization(ref e) => Some(e),
            Self::ObjectId(ref e) => Some(e),
            Self::MongoDb(ref e) => Some(e),
            Self::BadObjectId => None,
            Self::NoSuchMeeting => None,
        }
    }
}

/// Builds the MongoDB documents representing the update.
fn build_update(update_meeting: UpdateMeeting) -> Vec<Document> {
    let mut updates = Vec::new();

    // This is dumb. We could probably do something with serde to automatically
    // turn it into a Document.
    if let Some(value) = update_meeting.date {
        updates.push(doc! {"$set": {"date": value}})
    }
    if let Some(value) = update_meeting.location {
        updates.push(doc! {"$set": {"location": value}})
    }
    if let Some(value) = update_meeting.title {
        updates.push(doc! {"$set": {"title": value}})
    }
    if let Some(value) = update_meeting.author {
        updates.push(doc! {"$set": {"author": value}})
    }
    if let Some(value) = update_meeting.description {
        updates.push(doc! {"$set": {"description": value}})
    }
    if let Some(value) = update_meeting.pitched_by {
        updates.push(doc! {"$set": {"pitchedBy": value}})
    }
    if let Some(value) = update_meeting.first_suggested {
        updates.push(doc! {"$set": {"firstSuggested": value}})
    }
    if let Some(value) = update_meeting.supporters {
        updates.push(doc! {"$set": {"supporters": value}})
    }

    updates
}
