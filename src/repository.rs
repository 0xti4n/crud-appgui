use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use crate::models::{NewProperty, Property};
use crate::schema::property::dsl::*;

use std::env;

pub struct PopertyRepository {
    pub conn: SqliteConnection,
}

impl PopertyRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            conn: SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }

    pub fn create(&mut self, new_property: NewProperty) -> Result<Property, Error> {
        diesel::insert_into(property)
            .values(new_property)
            .execute(&mut self.conn)
            .expect("Error saving new post");

        property.order(id.desc()).first(&mut self.conn)
    }

    pub fn find_all(&mut self) -> Result<Vec<Property>, Error> {
        property.load::<Property>(&mut self.conn)
    }

    pub fn update(&mut self, item: Property) -> Result<Property, Error> {
        let item_id = item.id.clone().unwrap();
        diesel::update(property.filter(id.eq(item_id)))
            .set((
                street.eq(&item.street),
                number.eq(item.number),
                floor.eq(item.floor),
                postal_code.eq(item.postal_code),
                square_meters.eq(item.square_meters),
                num_bathrooms.eq(item.num_bathrooms),
                num_bedrooms.eq(item.num_bedrooms),
                dwelling_type.eq(item.dwelling_type),
            ))
            .execute(&mut self.conn)
            .unwrap();

        property.filter(id.eq(item.id)).first(&mut self.conn)
    }

    pub fn delete(&mut self, uniq_id: i32) -> Result<(), Error> {
        let _ = diesel::delete(property.filter(id.eq(uniq_id))).execute(&mut self.conn);
        Ok(())
    }
}
