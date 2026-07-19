use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};

use futures::future::BoxFuture;
use mongodb::{
    bson::{self, doc, Document},
    Collection, Database,
};
use serde::{de::DeserializeOwned, Serialize};
use teloxide_max_core::types::ChatId;
use thiserror::Error;

use super::{serializer::Serializer, Storage};

/// An error returned from [`MongoStorage`].
#[derive(Debug, Error)]
pub enum MongoStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),

    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("dialogue not found")]
    DialogueNotFound,
}

/// A persistent dialogue storage based on [MongoDB](https://www.mongodb.com/).
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max::dispatching::dialogue::{MongoStorage, Json};
///
/// # async fn run() {
/// let storage = MongoStorage::open(
///     "mongodb://localhost:27017",
///     "teloxide_max_db",
///     "dialogues",
///     Json,
/// )
/// .await
/// .unwrap();
/// # }
/// ```
pub struct MongoStorage<S> {
    collection: Collection<Document>,
    serializer: S,
}

impl<S> MongoStorage<S> {
    /// Opens a connection to the MongoDB database and creates the collection
    /// for storing dialogues.
    ///
    /// Parameters:
    /// - `uri`: MongoDB connection string, e.g. `"mongodb://localhost:27017"`
    /// - `db_name`: database name, e.g. `"teloxide_max_db"`
    /// - `collection_name`: collection name, e.g. `"dialogues"`
    /// - `serializer`: what [`Serializer`] will be used to encode the dialogue
    ///   data. Available ones are: [`Json`], [`Bincode`], [`Cbor`]
    ///
    /// [`Json`]: crate::dispatching::dialogue::serializer::Json
    /// [`Bincode`]: crate::dispatching::dialogue::serializer::Bincode
    /// [`Cbor`]: crate::dispatching::dialogue::serializer::Cbor
    pub async fn open(
        uri: &str,
        db_name: &str,
        collection_name: &str,
        serializer: S,
    ) -> Result<Arc<Self>, MongoStorageError<Infallible>> {
        let client = mongodb::Client::with_uri_str(uri).await?;
        let db: Database = client.database(db_name);
        let collection: Collection<Document> = db.collection(collection_name);

        // Create unique index on chat_id (equivalent to PRIMARY KEY in SQL)
        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "chat_id": 1_i32 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .unique(true)
                            .build(),
                    )
                    .build(),
            )
            .await?;

        Ok(Arc::new(Self { collection, serializer }))
    }

    /// Opens a connection using an existing MongoDB database.
    pub fn new(db: Database, collection_name: &str, serializer: S) -> Self {
        let collection: Collection<Document> = db.collection(collection_name);
        Self { collection, serializer }
    }
}

impl<S, D> Storage<D> for MongoStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = MongoStorageError<<S as Serializer<D>>::Error>;

    fn remove_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let result = self.collection.delete_one(doc! { "chat_id": chat_id }).await?;

            if result.deleted_count == 0 {
                return Err(MongoStorageError::DialogueNotFound);
            }

            Ok(())
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let bytes = self.serializer.serialize(&dialogue).map_err(MongoStorageError::SerdeError)?;

            // Upsert: insert if not exists, update if exists
            self.collection
                .update_one(
                    doc! { "chat_id": chat_id },
                    doc! { "$set": { "chat_id": chat_id, "dialogue": bson::Binary::from(bytes) } },
                )
                .with_options(
                    mongodb::options::UpdateOptions::builder()
                        .upsert(true)
                        .build(),
                )
                .await?;

            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            let result = self.collection.find_one(doc! { "chat_id": chat_id }).await?;

            match result {
                Some(doc) => {
                    let binary = match doc.get_binary("dialogue") {
                        Ok(b) => b,
                        Err(_) => return Ok(None),
                    };
                    let bytes = binary.bytes;
                    self.serializer
                        .deserialize(&bytes)
                        .map(Some)
                        .map_err(MongoStorageError::SerdeError)
                }
                None => Ok(None),
            }
        })
    }
}
