-- Your SQL goes here
CREATE TABLE property (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    street VARCHAR(255),
    number VARCHAR(10),
    floor VARCHAR(10),
    postal_code VARCHAR(10),
    square_meters INT,
    num_bathrooms INT,
    num_bedrooms INT,
    dwelling_type VARCHAR(50)
);
