#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
mod schema;
mod models;

use diesel::prelude::*;
use rocket::get;
use rocket::post;
use rocket::put;
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

use serde_derive::{Deserialize, Serialize};

use crate::schema::songs::dsl::songs;

pub use models::{NewSong, Song, UpdateSong};

pub fn establish_connection() -> SqliteConnection {
    let database_url = "songs.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[post("/song", format = "json", data = "<new_song>")]
pub fn create_song(new_song: Json<NewSong>) -> Json<JsonValue> {
    let connection = establish_connection();
    let genre_value = if let Some(genre) = &new_song.genre {
        Some(genre).cloned()
    } else {
        None
    };
    let new_song = NewSong {
        title: new_song.title.to_string(),
        artist: new_song.artist.to_string(),
        album: new_song.album.to_string(),
        year: new_song.year,
        genre: genre_value,
        times_played: new_song.times_played,
        last_played: new_song.last_played,
    };
    diesel::insert_into(crate::schema::songs::dsl::songs)
        .values(&new_song)
        .execute(&connection)
        .expect("Error creating new song");

    Json(JsonValue::from(json!({ "status": "ok", "message": "Song successfully created" })))
}

#[get("/songs")]
pub fn get_songs() -> Json<JsonValue> {
    let connection = establish_connection();
    let returned_songs = songs.load::<Song>(&connection).expect("Error loading songs");
    Json(JsonValue::from(json!({ "status": "ok", "result": returned_songs })))
}

#[put("/songs/<id>", data = "<update_song>")]
pub fn update_song(id: i32, update_song: Json<UpdateSong>) -> Json<JsonValue> {
    let connection = establish_connection();

    let _updated_song = diesel::update(songs.find(id))
        .set(update_song.into_inner())
        .execute(&connection)
        .expect("Error updating song");

    Json(JsonValue::from(json!({ "status": "ok", "message": "Song successfully updated" })))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            create_song, 
            get_songs, 
            update_song
            ]
        )
        .launch();
}
