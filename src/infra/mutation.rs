use super::post;
use crate::CreatePostRequest;
use sea_orm::ActiveModelTrait;
use sea_orm::DbConn;
use sea_orm::DbErr;
use sea_orm::Set;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        payload: CreatePostRequest,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(payload.title.to_owned()),
            text: Set(payload.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
