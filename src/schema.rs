// @generated automatically by Diesel CLI.

diesel::table! {
    puzzles (date) {
        date -> Text,
        solution -> Text,
        is_deleted -> Bool,
    }
}
