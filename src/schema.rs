// @generated automatically by Diesel CLI.

diesel::table! {
    users (userid) {
        userid -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        dob -> Nullable<Int4>,
        location -> Nullable<Varchar>,
    }
}
