use crate::schema::songs;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: i32,
    pub genre: Option<String>,
    pub times_played: i32,
    pub last_played: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "songs"]
pub struct NewSong {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: i32,
    pub genre: Option<String>,
    pub times_played: i32,
    pub last_played: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "songs"]
pub struct UpdateSong {
    pub times_played: i32,
    pub last_played: i32,
}
