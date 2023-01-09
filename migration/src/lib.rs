pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230103_030859_create_table_categories;
mod m20230103_133654_create_table_brand;
mod m20230105_095555_create_product_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230103_030859_create_table_categories::Migration),
            Box::new(m20230103_133654_create_table_brand::Migration),
            Box::new(m20230105_095555_create_product_table::Migration),
        ]
    }
}
