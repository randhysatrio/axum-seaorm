use chrono::Utc;
use migration::{Condition, Expr, Func};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, ItemsAndPagesNumber,
    PaginatorTrait, QueryFilter, Set,
};

use ::entity::{brand, prelude::Brand};

use super::{page_matcher, size_matcher};
use crate::errors::{APIResult, AppError};

pub struct BrandService;

impl BrandService {
    pub async fn create(db: &DbConn, name: String) -> APIResult<brand::Model> {
        if (Brand::find()
            .filter(brand::Column::Name.eq(name.as_str()))
            .one(db)
            .await?)
            .is_some()
        {
            return Err(AppError::DuplicateBrand);
        }

        Ok(brand::ActiveModel {
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
        page: Option<i32>,
        size: Option<i32>,
        all: Option<bool>,
    ) -> APIResult<(Vec<brand::Model>, u64, u64)> {
        let mut condition = Condition::all();

        if let Some(k) = keyword {
            let like = format!("%{}%", k.to_lowercase());

            condition =
                condition.add(Expr::expr(Func::lower(Expr::col(brand::Column::Name))).like(like));
        }

        if all.is_none() {
            condition = condition.add(brand::Column::DeletedAt.is_null());
        }

        let size = size_matcher(size)?;
        let page = page_matcher(page)?;

        let ItemsAndPagesNumber {
            number_of_items,
            number_of_pages,
        } = Brand::find()
            .filter(condition.clone())
            .paginate(db, size)
            .num_items_and_pages()
            .await?;
        let data = Brand::find()
            .filter(condition)
            .paginate(db, size)
            .fetch_page(page)
            .await?;

        Ok((data, number_of_items, number_of_pages))
    }

    pub async fn delete(db: &DbConn, id: i32) -> APIResult<()> {
        let brand = Brand::find_by_id(id).one(db).await?;

        let mut brand = if let Some(brand) = brand {
            if brand.deleted_at.is_none() {
                brand.into_active_model()
            } else {
                return Err(AppError::BrandAlreadyDeleted);
            }
        } else {
            return Err(AppError::BrandNotFound);
        };

        brand.deleted_at = Set(Some(Utc::now().into()));
        brand.update(db).await?;

        Ok(())
    }
}
