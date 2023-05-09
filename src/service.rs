use crate::models::{NewProperty, Property};
use crate::repository::PopertyRepository;
use diesel::result::Error;

pub struct PropertyService {
    pub repository: PopertyRepository,
}

impl PropertyService {
    pub fn new() -> Self {
        Self {
            repository: PopertyRepository::new(),
        }
    }

    pub fn create_post(&mut self, new_property: NewProperty) -> Result<Property, Error> {
        self.repository.create(new_property)
    }

    pub fn get_properties(&mut self) -> Result<Vec<Property>, diesel::result::Error> {
        self.repository.find_all()
    }

    pub fn update_property(
        &mut self,
        property: Property,
    ) -> Result<Property, diesel::result::Error> {
        self.repository.update(property.clone())
    }

    pub fn delete_property(&mut self, id: i32) -> Result<(), diesel::result::Error> {
        self.repository.delete(id)
    }
}
