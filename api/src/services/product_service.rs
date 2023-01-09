use chrono::Utc;
use migration::{Condition, Expr, Func};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, FromQueryResult, IntoActiveModel,
    ItemsAndPagesNumber, PaginatorTrait, QueryFilter, QuerySelect, Set,
};
use serde::Serialize;

use ::entity::{
    brand, category,
    prelude::{Brand, Category, Product},
    product,
};

use super::{page_matcher, size_matcher};
use crate::errors::{APIResult, AppError};

#[derive(Serialize, Debug, FromQueryResult)]
pub struct ProductData {
    id: i32,
    brand_id: i32,
    category_id: i32,
    name: String,
    description: Option<String>,
    price: i32,
    stock: i32,
    brand_name: String,
    category_name: String,
}

pub struct ProductService;

impl ProductService {
    pub async fn create(
        db: &DbConn,
        name: String,
        price: i32,
        stock: i32,
        category_id: i32,
        brand_id: i32,
        description: Option<String>,
    ) -> APIResult<product::Model> {
        if stock < 1 {
            return Err(AppError::InvalidStockAmount);
        }

        if price < 1 {
            return Err(AppError::InvalidPrice);
        }

        if (Category::find_by_id(category_id)
            .filter(category::Column::DeletedAt.is_null())
            .one(db)
            .await?)
            .is_none()
        {
            return Err(AppError::CategoryNotFound);
        }

        if (Brand::find_by_id(brand_id)
            .filter(brand::Column::DeletedAt.is_null())
            .one(db)
            .await?)
            .is_none()
        {
            return Err(AppError::BrandNotFound);
        }

        if (Product::find()
            .filter(product::Column::Name.eq(name.as_str()))
            .one(db)
            .await?)
            .is_some()
        {
            return Err(AppError::ProductAlreadyCreated);
        }

        Ok(product::ActiveModel {
            name: Set(name),
            price: Set(price),
            stock: Set(stock),
            category_id: Set(category_id),
            brand_id: Set(brand_id),
            description: Set(description),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        }
        .insert(db)
        .await?)
    }

    pub async fn find(
        db: &DbConn,
        keyword: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        all: Option<bool>,
    ) -> APIResult<(Vec<ProductData>, u64, u64)> {
        let mut condition = Condition::all();

        if let Some(l) = keyword {
            let like = format!("%{}%", l.to_lowercase());

            condition = condition.add(
                Expr::expr(Func::lower(Expr::col(
                    product::Column::Name.as_column_ref(),
                )))
                .like(like),
            );
        }

        if all.is_none() {
            condition = condition.add(product::Column::DeletedAt.is_null());
        }

        let size = size_matcher(size)?;
        let page = page_matcher(page)?;

        let ItemsAndPagesNumber {
            number_of_items,
            number_of_pages,
        } = Product::find()
            .filter(condition.clone())
            .left_join(Category)
            .left_join(Brand)
            .paginate(db, size)
            .num_items_and_pages()
            .await?;

        let data = Product::find()
            .select_only()
            .columns([
                product::Column::Id,
                product::Column::BrandId,
                product::Column::CategoryId,
                product::Column::Name,
                product::Column::Description,
                product::Column::Price,
                product::Column::Stock,
            ])
            .column_as(category::Column::Name, "category_name")
            .column_as(brand::Column::Name, "brand_name")
            .filter(condition)
            .left_join(Category)
            .left_join(Brand)
            .into_model::<ProductData>()
            .paginate(db, size)
            .fetch_page(page)
            .await?;

        Ok((data, number_of_items, number_of_pages))
    }

    pub async fn delete(db: &DbConn, id: i32) -> APIResult<()> {
        let mut product = if let Some(p) = Product::find_by_id(id).one(db).await? {
            if p.deleted_at.is_some() {
                return Err(AppError::ProductAlreadyDeleted);
            } else {
                p.into_active_model()
            }
        } else {
            return Err(AppError::ProductNotFound);
        };

        product.deleted_at = Set(Some(Utc::now().into()));
        product.update(db).await?;

        Ok(())
    }
}
