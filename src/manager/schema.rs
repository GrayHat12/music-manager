// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        name -> Text,
        image -> Nullable<Integer>,
        artist -> Integer,
        last_updated -> Integer,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        image -> Nullable<Integer>,
        last_updated -> Integer,
    }
}

diesel::table! {
    features (id) {
        id -> Integer,
        artist -> Integer,
        song -> Integer,
        last_updated -> Integer,
    }
}

diesel::table! {
    genre (id) {
        id -> Integer,
        last_updated -> Integer,
        name -> Text,
    }
}

diesel::table! {
    images (id) {
        id -> Integer,
        buffer -> Binary,
        last_updated -> Integer,
    }
}

diesel::table! {
    songs (id) {
        id -> Integer,
        genre -> Nullable<Integer>,
        artist -> Nullable<Integer>,
        album -> Nullable<Integer>,
        cover -> Nullable<Integer>,
        title -> Text,
        release -> Nullable<Integer>,
        trackno -> Nullable<Integer>,
        metatags -> Text,
        buffer -> Binary,
        last_updated -> Integer,
    }
}

diesel::joinable!(albums -> artists (artist));
diesel::joinable!(albums -> images (image));
diesel::joinable!(artists -> images (image));
diesel::joinable!(features -> artists (artist));
diesel::joinable!(features -> songs (song));
diesel::joinable!(songs -> albums (album));
diesel::joinable!(songs -> artists (artist));
diesel::joinable!(songs -> genre (genre));
diesel::joinable!(songs -> images (cover));

diesel::allow_tables_to_appear_in_same_query!(albums, artists, features, genre, images, songs,);
