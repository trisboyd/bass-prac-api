// @generated automatically by Diesel CLI.

diesel::table! {
    songs (id) {
        id -> Integer,
        title -> Text,
        artist -> Text,
        album -> Text,
        year -> Integer,
        genre -> Nullable<Text>,
        times_played -> Integer,
        last_played -> Integer,
    }
}
