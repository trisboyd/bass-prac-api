-- Your SQL goes here
CREATE TABLE songs (
    id integer primary key not null,
    title text not null,
    artist text not null,
    album text not null,
    year integer not null,
    genre text,
    times_played integer not null,
    last_played integer not null
);