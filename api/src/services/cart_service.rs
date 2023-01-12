use chrono::Utc;
use migration::{Condition, Expr, JoinType};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait,
    FromQueryResult, IntoActiveModel, ItemsAndPagesNumber, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use serde::Serialize;

use ::entity::{
    brand, cart, category,
    prelude::{Cart, Product, User},
    product,
};

use super::{page_matcher, size_matcher};
use crate::errors::{APIResult, AppError};

#[derive(Debug, Serialize, FromQueryResult)]
pub struct CartData {
    id: i32,
    quantity: i32,
    product_id: i32,
    product_category: String,
    product_brand: String,
    product_name: String,
    product_price: i32,
    product_stock: i32,
    subtotal: i32,
    product_deleted_at: Option<DateTimeWithTimeZone>,
    brand_deleted_at: Option<DateTimeWithTimeZone>,
    category_deleted_at: Option<DateTimeWithTimeZone>,
}

pub struct CartService;

impl CartService {
    pub async fn create_or_update(
        db: &DbConn,
        user_id: i32,
        product_id: i32,
        quantity: i32,
    ) -> APIResult<&'static str> {
        let cart_product = Product::find_by_id(product_id).one(db).await?;

        if cart_product.is_none() {
            return Err(AppError::ProductNotFound);
        }

        let cart_product = cart_product.unwrap();

        if quantity < 1 {
            return Err(AppError::InvalidQuantity);
        }

        if quantity > cart_product.stock {
            return Err(AppError::InsufficientStock);
        }

        if (User::find_by_id(user_id).one(db).await?).is_none() {
            return Err(AppError::UserNotFound);
        }

        let condition = Condition::all()
            .add(Expr::col(cart::Column::UserId).eq(user_id))
            .add(Expr::col(cart::Column::ProductId).eq(product_id));

        let user_cart = Cart::find().filter(condition).one(db).await?;

        if user_cart.is_none() {
            cart::ActiveModel {
                user_id: Set(user_id),
                product_id: Set(product_id),
                quantity: Set(quantity),
                created_at: Set(Utc::now().into()),
                updated_at: Set(Utc::now().into()),
                ..Default::default()
            }
            .insert(db)
            .await?;

            Ok("Cart created successfully!")
        } else {
            let mut user_cart = user_cart.unwrap().into_active_model();

            user_cart.quantity = Set(quantity);
            user_cart.updated_at = Set(Utc::now().into());
            user_cart.update(db).await?;

            Ok("Cart updated successfully!")
        }
    }

    pub async fn get(
        db: &DbConn,
        user_id: i32,
        page: Option<i32>,
        size: Option<i32>,
    ) -> APIResult<(Vec<CartData>, u64, u64)> {
        let page = page_matcher(page)?;
        let size = size_matcher(size)?;

        let ItemsAndPagesNumber {
            number_of_items,
            number_of_pages,
        } = Cart::find()
            .filter(cart::Column::UserId.eq(user_id))
            .left_join(Product)
            .join_rev(JoinType::LeftJoin, category::Relation::Product.def())
            .join_rev(JoinType::LeftJoin, brand::Relation::Product.def())
            .paginate(db, size)
            .num_items_and_pages()
            .await?;
        let data = Cart::find()
            .filter(cart::Column::UserId.eq(user_id))
            .left_join(Product)
            .column_as(product::Column::Name, "product_name")
            .column_as(product::Column::Price, "product_price")
            .column_as(product::Column::Stock, "product_stock")
            .join_rev(JoinType::LeftJoin, category::Relation::Product.def())
            .column_as(category::Column::Name, "product_category")
            .join_rev(JoinType::LeftJoin, brand::Relation::Product.def())
            .column_as(brand::Column::Name, "product_brand")
            .column_as(Expr::cust("(cart.quantity * product.price)"), "subtotal")
            .column_as(product::Column::DeletedAt, "product_deleted_at")
            .column_as(brand::Column::DeletedAt, "brand_deleted_at")
            .column_as(category::Column::DeletedAt, "category_deleted_at")
            .into_model::<CartData>()
            .paginate(db, size)
            .fetch_page(page)
            .await?;

        Ok((data, number_of_items, number_of_pages))
    }
}
