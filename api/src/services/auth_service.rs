use ::entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

use crate::dto::{APIResult, Error};
use crate::utils::encryption::validate_password;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(
        db: &DbConn,
        username: String,
        email: String,
        password: String,
    ) -> APIResult<user::Model> {
        if let Some(_) = User::find()
            .filter(user::Column::Username.eq(username.as_str()))
            .one(db)
            .await?
        {
            return Err(Error::DuplicateUsername);
        }

        if let Some(_) = User::find()
            .filter(user::Column::Email.eq(email.as_str()))
            .one(db)
            .await?
        {
            return Err(Error::DuplicateEmail);
        }

        user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set(password),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| Error::DBError(e))
    }

    pub async fn login_user(
        db: &DbConn,
        email: String,
        password: String,
    ) -> APIResult<user::Model> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;

        if let Some(user) = user {
            let valid_user = validate_password(password, &user.password);

            if !valid_user {
                return Err(Error::WrongCredentials);
            } else {
                Ok(user)
            }
        } else {
            return Err(Error::WrongCredentials);
        }
    }
}
