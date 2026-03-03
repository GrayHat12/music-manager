use std::collections::HashMap;
use std::io::Read;

use crate::{library, types::*};
use diesel::SqliteConnection;
use diesel::prelude::*;
use music_manager::models::*;
use music_manager::schema::*;
use symphonia::core::meta::StandardTagKey;

fn get_now() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

fn tags_map_to_array(tags: HashMap<StandardTagKey, String>) -> [String; 111] {
    let mut values: [String; 111] = core::array::from_fn(|_| "".to_string());
    for (tag, value) in tags {
        match tag.from_enum() {
            Some(index) => values[index] = value,
            None => {}
        };
    }
    values
}

pub fn add_song(connection: &mut SqliteConnection, command: Add) -> Song {
    let mut song_data =
        library::load_song(command.path.clone()).expect("Failed to load song from path");

    // println!("got song {:?}", song_data);

    if let Some(g) = command.genre {
        song_data.genre = Some(g);
    }
    if let Some(a) = command.artist {
        song_data.artist = Some(a);
    }
    if let Some(al) = command.album {
        song_data.album = Some(al);
    }
    if let Some(t) = command.title {
        song_data.title = t;
    }
    if let Some(r) = command.release {
        song_data.release = Some(r.to_string());
    }
    if let Some(i) = command.index {
        song_data.trackno = Some(i as i32);
    }
    if !command.features.is_empty() {
        song_data.features = command.features;
    }

    let genre_id: Option<i32> = song_data.genre.map(|name| {
        diesel::insert_into(genre::table)
            .values((genre::name.eq(&name), genre::last_updated.eq(get_now())))
            .on_conflict(genre::name)
            .do_nothing()
            .execute(connection)
            .expect("failed genre insert");
        genre::table
            .filter(genre::name.eq(name))
            .select(genre::id)
            .first::<i32>(connection)
            .expect("failed genre fetch")
    });

    let artist_id = song_data.artist.map(|name| {
        diesel::insert_into(artists::table)
            .values((artists::name.eq(&name), artists::last_updated.eq(get_now())))
            .on_conflict(artists::name)
            .do_nothing()
            .execute(connection)
            .expect("failed artist insert");
        artists::table
            .filter(artists::name.eq(name))
            .select(artists::id)
            .first::<i32>(connection)
            .expect("failed artist fetch")
    });

    let album_id = song_data.album.map(|name| {
        diesel::insert_into(albums::table)
            .values((
                albums::name.eq(&name),
                albums::artist.eq(artist_id.unwrap_or(0)),
                albums::last_updated.eq(get_now()),
            ))
            .on_conflict((albums::name, albums::artist))
            .do_nothing()
            .execute(connection)
            .expect("failed album insert");
        albums::table
            .filter(albums::name.eq(name))
            .select(albums::id)
            .first::<i32>(connection)
            .expect("failed album fetch")
    });

    let mut features = Vec::new();

    for feature in song_data.features {
        features.push({
            diesel::insert_into(artists::table)
                .values((
                    artists::name.eq(&feature),
                    artists::last_updated.eq(get_now()),
                ))
                .on_conflict(artists::name)
                .do_nothing()
                .execute(connection)
                .expect("failed artist insert");
            artists::table
                .filter(artists::name.eq(feature))
                .select(artists::id)
                .first::<i32>(connection)
                .expect("failed artist fetch")
        });
    }

    let tags_array = tags_map_to_array(song_data.tags).to_vec();

    // 4. Final Song Insertion
    let mut new_song = NewSong {
        genre: genre_id,
        artist: artist_id,
        album: album_id,
        cover: None, // Logic for image buffer extraction could be added here
        title: song_data.title,
        release: song_data.release.and_then(|r| r.parse::<i32>().ok()),
        trackno: song_data.trackno,
        metatags: match serde_json::to_string(&tags_array) {
            Ok(v) => v,
            Err(_) => "[]".to_string(),
        },
        buffer: song_data.buffer,
        last_updated: get_now(),
    };

    if let Some(image_path) = command.art {
        let mut src = std::fs::File::open(image_path).expect("failed to open image path");
        let mut buffer = Vec::new();
        src.read_to_end(&mut buffer).expect("Failed reading file");

        new_song.cover = Some({
            let id = diesel::insert_into(images::table)
                .values(NewImage {
                    buffer,
                    last_updated: get_now(),
                })
                .returning(images::id)
                .on_conflict(images::buffer)
                .do_nothing()
                .get_result::<i32>(connection)
                .expect("failed image insert");
            images::table
                .filter(images::id.eq(id))
                .select(images::id)
                .first::<i32>(connection)
                .expect("failed album fetch")
        });
    }

    println!("adding song");

    let songid = diesel::insert_into(songs::table)
        .values(&new_song)
        .returning(songs::id)
        .get_result(connection)
        .expect("Error saving song to database");

    for feature in features {
        diesel::insert_into(features::table)
            .values(NewFeature {
                artist: feature,
                song: songid,
                last_updated: get_now(),
            })
            .on_conflict((features::artist, features::song))
            .do_nothing()
            .execute(connection)
            .expect("failed feature insert");
    }

    let song: Song = songs::table
        .filter(songs::id.eq(songid))
        .first::<Song>(connection)
        .expect("Error fething song");

    println!("Successfully added: {}", new_song.title);

    return song;
}

pub fn remove_song(connection: &mut SqliteConnection, command: Remove) -> String {
    let id_int = command.id.parse::<i32>().expect("ID must be an integer");
    let title = diesel::delete(songs::table.filter(songs::id.eq(id_int)))
        .returning(songs::title)
        .get_result::<String>(connection)
        .expect("Error deleting song");
    println!("Removed song ID: {} {}", id_int, title);
    return title;
}

pub fn list_songs(connection: &mut SqliteConnection, command: List) -> Vec<SongLite> {
    let mut query = songs::table.into_boxed().select(SongLite::as_select());

    if !command.title.is_empty() {
        for pattern in command.title {
            query = query.filter(songs::title.like(pattern));
        }
    }

    if !command.genre.is_empty() {
        let mut genre_query = genre::table.into_boxed().select(genre::id);
        for genre in command.genre {
            genre_query = genre_query.filter(genre::name.like(genre));
        }
        let genreids = genre_query
            .load::<i32>(connection)
            .expect("Error fetching genre");
        query = query.filter(songs::genre.eq_any(genreids));
    }

    if !command.artist.is_empty() {
        let mut artist_query = artists::table.into_boxed().select(artists::id);
        for artist in command.artist {
            artist_query = artist_query.filter(artists::name.like(artist));
        }
        let artistids = artist_query
            .load::<i32>(connection)
            .expect("Error fetching artists");
        query = query.filter(songs::artist.eq_any(artistids));
    }

    if !command.features.is_empty() {
        let mut artist_query = artists::table.into_boxed().select(artists::id);
        for artist in command.features {
            artist_query = artist_query.filter(artists::name.like(artist));
        }
        let artistids = artist_query
            .load::<i32>(connection)
            .expect("Error fetching artists");
        let featureids = features::table
            .into_boxed()
            .select(features::song)
            .filter(features::artist.eq_any(artistids))
            .load::<i32>(connection)
            .expect("Error fetching features");
        query = query.filter(songs::id.eq_any(featureids));
    }

    if !command.album.is_empty() {
        let mut album_query = albums::table.into_boxed().select(albums::id);
        for album in command.album {
            album_query = album_query.filter(albums::name.like(album));
        }
        let albumids = album_query
            .load::<i32>(connection)
            .expect("Error fetching albums");
        query = query.filter(songs::album.eq_any(albumids));
    }

    if !command.release.is_empty() {
        query = query.filter(songs::release.eq_any(command.release.iter().map(|x| *x as i32)));
    }

    if !command.index.is_empty() {
        query = query.filter(songs::trackno.eq_any(command.index.iter().map(|x| *x as i32)));
    }

    let results = query
        .load::<SongLite>(connection)
        .expect("Error loading songs");
    for s in &results {
        println!("{}: {}", s.id, s.title);
    }
    return results;
}

pub fn list_artists(connection: &mut SqliteConnection, _command: ListArtists) -> Vec<Artist> {
    let mut query = artists::table.into_boxed();
    if !_command.album.is_empty() {
        let mut album_query = albums::table.into_boxed().select(albums::artist);
        for album in _command.album {
            album_query = album_query.filter(albums::name.like(album));
        }
        let artistids = album_query
            .load::<i32>(connection)
            .expect("Error loading albums");
        query = query.filter(artists::id.eq_any(artistids));
    }
    if !_command.genre.is_empty() {
        let mut genre_query = genre::table.into_boxed().select(genre::id);
        for genre in _command.genre {
            genre_query = genre_query.filter(genre::name.like(genre));
        }
        let genreids = genre_query
            .load::<i32>(connection)
            .expect("Error loading genres");
        let artistids = songs::table
            .select(songs::artist)
            .filter(songs::genre.eq_any(genreids))
            .load::<Option<i32>>(connection)
            .expect("Error loading albums");
        query = query.filter(
            artists::id.eq_any(artistids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())),
        );
    }
    if !_command.release.is_empty() {
        let artistids = songs::table
            .select(songs::artist)
            .filter(songs::release.eq_any(_command.release.iter().map(|x| *x as i32)))
            .load::<Option<i32>>(connection)
            .expect("Error loading songs");
        query = query.filter(
            artists::id.eq_any(artistids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())),
        );
    }
    let results = query
        .load::<Artist>(connection)
        .expect("Error loading artists");
    for a in &results {
        println!("{}: {}", a.id, a.name);
    }
    return results;
}

pub fn list_albums(connection: &mut SqliteConnection, _command: ListAlbums) -> Vec<Album> {
    let mut query = albums::table.into_boxed();
    if !_command.artist.is_empty() {
        let mut artistquery = artists::table.into_boxed().select(artists::id);
        for artist in _command.artist {
            artistquery = artistquery.filter(artists::name.like(artist));
        }
        let albumids = songs::table
            .select(songs::album)
            .filter(
                songs::artist.eq_any(
                    artistquery
                        .load::<i32>(connection)
                        .expect("failed loading artists"),
                ),
            )
            .load::<Option<i32>>(connection)
            .expect("Failed loading albums");
        query = query
            .filter(albums::id.eq_any(albumids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())));
    }
    if !_command.genre.is_empty() {
        let mut genrequery = genre::table.into_boxed().select(genre::id);
        for genre in _command.genre {
            genrequery = genrequery.filter(genre::name.like(genre));
        }
        let albumids = songs::table
            .select(songs::album)
            .filter(
                songs::genre.eq_any(
                    genrequery
                        .load::<i32>(connection)
                        .expect("failed loading artists"),
                ),
            )
            .load::<Option<i32>>(connection)
            .expect("Failed loading albums");
        query = query
            .filter(albums::id.eq_any(albumids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())));
    }
    if !_command.release.is_empty() {
        let albumids: Vec<Option<i32>> = songs::table
            .select(songs::album)
            .filter(songs::release.eq_any(_command.release.iter().map(|x| *x as i32)))
            .load::<Option<i32>>(connection)
            .expect("Failed loading albums");
        query = query
            .filter(albums::id.eq_any(albumids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())));
    }
    let results = query
        .load::<Album>(connection)
        .expect("Error loading albums");
    for a in &results {
        println!("{}: {}", a.id, a.name);
    }
    return results;
}

pub fn list_genres(connection: &mut SqliteConnection, _command: ListGenres) -> Vec<Genre> {
    let mut query = genre::table.into_boxed();
    if !_command.album.is_empty() {
        let mut albumquery = albums::table.into_boxed().select(albums::id);
        for album in _command.album {
            albumquery = albumquery.filter(albums::name.like(album));
        }
        let genreids = songs::table
            .select(songs::genre)
            .filter(
                songs::album.eq_any(
                    albumquery
                        .load::<i32>(connection)
                        .expect("failed loading artists"),
                ),
            )
            .load::<Option<i32>>(connection)
            .expect("Failed loading albums");
        query = query
            .filter(genre::id.eq_any(genreids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())));
    }
    if !_command.release.is_empty() {
        let genreids: Vec<Option<i32>> = songs::table
            .select(songs::genre)
            .filter(songs::release.eq_any(_command.release.iter().map(|x| *x as i32)))
            .load::<Option<i32>>(connection)
            .expect("Failed loading albums");
        query = query
            .filter(genre::id.eq_any(genreids.iter().filter(|p| p.is_some()).map(|p| p.unwrap())));
    }
    let results = query
        .load::<Genre>(connection)
        .expect("Error loading albums");
    for g in &results {
        println!("{}: {}", g.id, g.name);
    }
    return results;
}
