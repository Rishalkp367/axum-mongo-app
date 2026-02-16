use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Collection,
};

use crate::models::user_model::{CreateUserRequest, UpdateUserRequest, User};

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &mongodb::Database) -> Self {
        Self {
            collection: db.collection::<User>("users"),
        }
    }

    pub async fn create(&self, payload: CreateUserRequest) -> mongodb::error::Result<User> {
        let user = User {
            id: None,
            name: payload.name,
            email: payload.email,
            created_at: Some(DateTime::now()),
        };

        let result = self.collection.insert_one(&user, None).await?;

        let mut inserted_user = user;
        inserted_user.id = result.inserted_id.as_object_id();

        Ok(inserted_user)
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<User>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }

    pub async fn find_by_id(&self, id: &str) -> mongodb::error::Result<Option<User>> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ObjectId",
            ))
        })?;

        self.collection.find_one(doc! { "_id": obj_id }, None).await
    }

    pub async fn update(&self, id: &str, payload: UpdateUserRequest) -> mongodb::error::Result<()> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ObjectId",
            ))
        })?;

        let mut update_doc = doc! {};

        if let Some(name) = payload.name {
            update_doc.insert("name", name);
        }

        if let Some(email) = payload.email {
            update_doc.insert("email", email);
        }

        self.collection
            .update_one(doc! { "_id": obj_id }, doc! { "$set": update_doc }, None)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> mongodb::error::Result<()> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ObjectId",
            ))
        })?;

        self.collection
            .delete_one(doc! { "_id": obj_id }, None)
            .await?;
        Ok(())
    }
}
