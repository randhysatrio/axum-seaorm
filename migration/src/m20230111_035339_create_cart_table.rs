use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::User, m20230105_095555_create_product_table::Product};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cart::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cart::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cart-user-id")
                            .from(Cart::Table, Cart::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Cart::ProductId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cart-product-id")
                            .from(Cart::Table, Cart::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .col(ColumnDef::new(Cart::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Cart::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cart::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Cart {
    Table,
    Id,
    UserId,
    ProductId,
    Quantity,
    CreatedAt,
    UpdatedAt,
}
