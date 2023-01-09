use sea_orm_migration::prelude::*;

use crate::{
    m20230103_030859_create_table_categories::Category, m20230103_133654_create_table_brand::Brand,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).string().null())
                    .col(ColumnDef::new(Product::Price).integer().not_null())
                    .col(ColumnDef::new(Product::Stock).integer().not_null())
                    .col(ColumnDef::new(Product::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_id")
                            .from(Product::Table, Product::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Product::BrandId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_brand_id")
                            .from(Product::Table, Product::BrandId)
                            .to(Brand::Table, Brand::Id),
                    )
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Product::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Name,
    Description,
    Price,
    Stock,
    CategoryId,
    BrandId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
