// @generated automatically by Diesel CLI.

diesel::table! {
    puzzles (date) {
        date -> Text,
        puzzle -> Text,
        is_deleted -> Bool,
    }
}
