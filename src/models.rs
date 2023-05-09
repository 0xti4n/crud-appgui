use diesel::{FromSqlRow, Insertable, Queryable, Selectable};

use crate::schema::property;

#[derive(Debug, Queryable, Selectable, Clone)]
#[diesel(table_name = property)]
pub struct Property {
    pub id: Option<i32>,
    pub street: Option<String>,
    pub number: Option<String>,
    pub floor: Option<String>,
    pub postal_code: Option<String>,
    pub square_meters: Option<i32>,
    pub num_bathrooms: Option<i32>,
    pub num_bedrooms: Option<i32>,
    pub dwelling_type: Option<String>,
}

#[derive(Debug, Insertable, FromSqlRow)]
#[diesel(table_name = property)]
pub struct NewProperty {
    pub street: String,
    pub number: String,
    pub floor: String,
    pub postal_code: String,
    pub square_meters: i32,
    pub num_bathrooms: i32,
    pub num_bedrooms: i32,
    pub dwelling_type: String,
}

pub trait ScreenOutput {
    fn to_screen(&self) -> String;
}

impl ScreenOutput for Property {
    fn to_screen(&self) -> String {
        format!("Id:{:?},Street:{:?},Number:{:?},Floor:{:?},Postal code:{:?},Number:{:?},# Bathrooms{:?},# Bedrooms{:?},Type:{:?}\n",
        self.id,
        self.street,
        self.number,
        self.floor,
        self.postal_code,
        self.number,
        self.num_bathrooms,
        self.num_bedrooms,
        self.dwelling_type)
    }
}
