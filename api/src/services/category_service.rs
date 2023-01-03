use chrono::Utc;
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QuerySelect, Set, Value,
};

use ::entity::{category, prelude::Category};

use crate::errors::{APIResult, AppError};

pub struct CategoryService;

impl CategoryService {
    pub async fn create(db: &DbConn, name: String) -> APIResult<category::Model> {
        if let Some(_) = Category::find()
            .filter(category::Column::Name.eq(name.as_str()))
            .one(db)
            .await?
        {
            return Err(AppError::DuplicateCategory);
        }

        Ok(category::ActiveModel {
            name: Set(name),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        }
        .insert(db)
        .await?)
    }

    pub async fn get(
        db: &DbConn,
        keyword: Option<String>,
        all: Option<bool>,
        page: Option<u64>,
        size: Option<u64>,
    ) -> APIResult<(u64, Vec<category::Model>)> {
        let mut condition = Condition::all();

        if let Some(keyword) = keyword {
            condition = condition.add(category::Column::Name.contains(keyword.as_str()));
        }

        if let None = all {
            condition = condition.add(category::Column::DeletedAt.is_null());
        }

        let page = page.unwrap_or_else(|| 1);
        let size = size.unwrap_or_else(|| 10);
        let offset = size * page - size;

        let count = Category::find().filter(condition.clone()).count(db).await?;
        let data = Category::find()
            .filter(condition)
            .offset(offset)
            .limit(size)
            .all(db)
            .await?;

        Ok((count, data))
    }

    pub async fn delete(db: &DbConn, id: i32) -> APIResult<()> {
        let category = Category::find_by_id(id).one(db).await?;

        let mut category = if let Some(category) = category {
            category.into_active_model()
        } else {
            return Err(AppError::CategoryNotFound);
        };

        if let Value::ChronoDateTimeWithTimeZone(Some(_)) =
            category.deleted_at.into_value().unwrap()
        {
            return Err(AppError::CategoryAlreadyDeleted);
        };

        category.deleted_at = Set(Some(Utc::now().into()));

        category.update(db).await?;

        Ok(())
    }

    pub async fn hard_delete(db: &DbConn, id: i32) -> APIResult<String> {
        Category::delete_by_id(id).exec(db).await?;

        Ok("Category deleted successfully".to_string())
    }
}
