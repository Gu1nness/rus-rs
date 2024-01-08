// @generated automatically by Diesel CLI.

diesel::table! {
    urls (short) {
        #[max_length = 24]
        short -> Varchar,
        #[max_length = 255]
        destination -> Varchar,
    }
}
