use chrono::Utc;
use migration::{Condition, Expr, Func, IntoCondition, JoinType};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, FromQueryResult, IntoActiveModel,
    ItemsAndPagesNumber, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
};
use serde::Serialize;

use ::entity::{
    brand, category,
    prelude::{Brand, Category, Product},
    product,
};

use super::{page_matcher, size_matcher};
use crate::{
    errors::{APIResult, AppError},
    handler::product::UpdateProductData,
};

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
            return Err(AppError::InvalidStock);
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
            .join(
                JoinType::LeftJoin,
                product::Relation::Category
                    .def()
                    .on_condition(|_left_t, right_t| {
                        Expr::tbl(right_t, category::Column::DeletedAt)
                            .is_null()
                            .into_condition()
                    }),
            )
            .join(
                JoinType::LeftJoin,
                product::Relation::Brand
                    .def()
                    .on_condition(|_left_t, right_t| {
                        Expr::tbl(right_t, brand::Column::DeletedAt)
                            .is_null()
                            .into_condition()
                    }),
            )
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
            .join(
                JoinType::LeftJoin,
                product::Relation::Category
                    .def()
                    .on_condition(|_left_t, right_t| {
                        Expr::tbl(right_t, category::Column::DeletedAt)
                            .is_null()
                            .into_condition()
                    }),
            )
            .join(
                JoinType::LeftJoin,
                product::Relation::Brand
                    .def()
                    .on_condition(|_left_t, right_t| {
                        Expr::tbl(right_t, brand::Column::DeletedAt)
                            .is_null()
                            .into_condition()
                    }),
            )
            .into_model::<ProductData>()
            .paginate(db, size)
            .fetch_page(page)
            .await?;

        Ok((data, number_of_items, number_of_pages))
    }

    pub async fn update(db: &DbConn, id: i32, update_data: UpdateProductData) -> APIResult<()> {
        let UpdateProductData {
            name,
            price,
            stock,
            description,
            category_id,
            brand_id,
        } = update_data;

        let mut product = if let Some(p) = Product::find_by_id(id).one(db).await? {
            if p.deleted_at.is_some() {
                return Err(AppError::ProductAlreadyDeleted);
            } else {
                p.into_active_model()
            }
        } else {
            return Err(AppError::ProductNotFound);
        };

        if let Some(c) = category_id {
            if (Category::find_by_id(c)
                .filter(brand::Column::DeletedAt.is_null())
                .one(db)
                .await?)
                .is_none()
            {
                return Err(AppError::CategoryNotFound);
            } else {
                product.category_id = Set(c);
            }
        }

        if let Some(b) = brand_id {
            if (Brand::find_by_id(b)
                .filter(brand::Column::DeletedAt.is_null())
                .one(db)
                .await?)
                .is_none()
            {
                return Err(AppError::BrandNotFound);
            } else {
                product.brand_id = Set(b);
            }
        }

        if let Some(n) = name {
            product.name = Set(n);
        }

        if let Some(p) = price {
            if p < 1 {
                return Err(AppError::InvalidPrice);
            } else {
                product.price = Set(p);
            }
        }

        if let Some(s) = stock {
            if s < 1 {
                return Err(AppError::InvalidStock);
            } else {
                product.stock = Set(s);
            }
        }

        if description.is_some() {
            product.description = Set(description);
        }

        product.updated_at = Set(Utc::now().into());
        product.update(db).await?;

        Ok(())
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

    pub async fn restore(db: &DbConn, id: i32) -> APIResult<()> {
        let mut product = if let Some(p) = Product::find_by_id(id).one(db).await? {
            if p.deleted_at.is_none() {
                return Err(AppError::CannotRestoreProduct);
            } else {
                p.into_active_model()
            }
        } else {
            return Err(AppError::ProductNotFound);
        };

        product.deleted_at = Set(None);
        product.update(db).await?;

        Ok(())
    }
}
