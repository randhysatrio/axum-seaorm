use chrono::Utc;
use migration::{Condition, Expr, Func};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, ItemsAndPagesNumber,
    PaginatorTrait, QueryFilter, Set,
};

use ::entity::{category, prelude::Category};

use super::{page_matcher, size_matcher};
use crate::errors::{APIResult, AppError};

pub struct CategoryService;

impl CategoryService {
    pub async fn create(db: &DbConn, name: String) -> APIResult<category::Model> {
        if (Category::find()
            .filter(category::Column::Name.eq(name.as_str()))
            .one(db)
            .await?)
            .is_some()
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
        page: Option<i32>,
        size: Option<i32>,
    ) -> APIResult<(Vec<category::Model>, u64, u64)> {
        let mut condition = Condition::all();

        if let Some(k) = keyword {
            let like = format!("%{}%", k.to_lowercase());

            condition = condition
                .add(Expr::expr(Func::lower(Expr::col(category::Column::Name))).like(like));
        }

        if all.is_none() {
            condition = condition.add(category::Column::DeletedAt.is_null());
        }

        let size = size_matcher(size)?;
        let page = page_matcher(page)?;

        let ItemsAndPagesNumber {
            number_of_items,
            number_of_pages,
        } = Category::find()
            .filter(condition.clone())
            .paginate(db, size)
            .num_items_and_pages()
            .await?;
        let data = Category::find()
            .filter(condition)
            .paginate(db, size)
            .fetch_page(page)
            .await?;

        Ok((data, number_of_items, number_of_pages))
    }

    pub async fn delete(db: &DbConn, id: i32) -> APIResult<()> {
        let category = Category::find_by_id(id).one(db).await?;

        let mut category = if let Some(c) = category {
            if c.deleted_at.is_none() {
                c.into_active_model()
            } else {
                return Err(AppError::CategoryAlreadyDeleted);
            }
        } else {
            return Err(AppError::CategoryNotFound);
        };

        category.deleted_at = Set(Some(Utc::now().into()));
        category.update(db).await?;

        Ok(())
    }

    #[allow(dead_code)] // for now;
    pub async fn hard_delete(db: &DbConn, id: i32) -> APIResult<String> {
        Category::delete_by_id(id).exec(db).await?;

        Ok("Category deleted successfully".to_string())
    }

    pub async fn restore(db: &DbConn, id: i32) -> APIResult<()> {
        let category = Category::find_by_id(id).one(db).await?;

        let mut category = if let Some(c) = category {
            if c.deleted_at.is_none() {
                return Err(AppError::CannotRestoreCategory);
            } else {
                c.into_active_model()
            }
        } else {
            return Err(AppError::CategoryNotFound);
        };

        category.deleted_at = Set(None);
        category.update(db).await?;

        Ok(())
    }
}
