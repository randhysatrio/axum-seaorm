use ::entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

pub struct AuthMutation;

impl AuthMutation {
    pub async fn register_user(
        db: &DbConn,
        username: String,
        email: String,
        password: String,
    ) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set(password),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
