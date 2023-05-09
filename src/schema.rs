// @generated automatically by Diesel CLI.

diesel::table! {
    property (id) {
        id -> Nullable<Integer>,
        street -> Nullable<Text>,
        number -> Nullable<Text>,
        floor -> Nullable<Text>,
        postal_code -> Nullable<Text>,
        square_meters -> Nullable<Integer>,
        num_bathrooms -> Nullable<Integer>,
        num_bedrooms -> Nullable<Integer>,
        dwelling_type -> Nullable<Text>,
    }
}
