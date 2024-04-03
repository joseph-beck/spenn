// @generated automatically by Diesel CLI.

diesel::table! {
    macs (uuid) {
        uuid -> Binary,
        address -> Text,
    }
}