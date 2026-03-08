use std::collections::HashMap;
use std::io::Read;

use diesel::SqliteConnection;
use diesel::prelude::*;
use symphonia::core::meta::StandardTagKey;

use crate::manager::models::*;
use crate::manager::schema::albums;
use crate::manager::schema::artists;
use crate::manager::schema::features;
use crate::manager::schema::genre;
use crate::manager::schema::playlistref;
use crate::manager::schema::playlists;
use crate::manager::schema::songs;
use crate::manager::types::*;
use crate::manager::utils::load_song;

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
    let mut song_data = load_song(command.path.clone()).expect("Failed to load song from path");

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

    let genre_id = match song_data.genre {
        Some(name) => Some(Genre::new(connection, &name).id),
        None => None,
    };

    let artist_id = match song_data.artist {
        Some(name) => match Artist::from_name(connection, &name) {
            Ok(artist) => Some(artist.id),
            Err(_) => None,
        },
        None => None,
    };

    let album_id = match song_data.album {
        Some(album_name) => match artist_id {
            Some(id) => match Artist::from_id(connection, id) {
                Ok(artist) => match Album::from_name(connection, &album_name, &artist.name) {
                    Ok(album) => Some(album.id),
                    Err(_) => None,
                },
                Err(_) => None,
            },
            None => None,
        },
        None => None,
    };

    let mut features = Vec::new();

    for feature in song_data.features {
        match Artist::from_name(connection, &feature) {
            Ok(artist) => features.push(artist.id),
            Err(_) => (),
        };
    }

    let tags_array = tags_map_to_array(song_data.tags).to_vec();

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
        new_song.cover = Some(CoverImage::new(connection, &buffer).id);
    }

    println!("adding song");

    let song = Song::new(connection, &new_song);

    _ = song.add_features(connection, &features);

    println!("Successfully added: {}", song.title);

    return song;
}

pub fn list_songs(connection: &mut SqliteConnection, command: List) -> Vec<Song> {
    let mut query = songs::table.into_boxed().select(Song::as_select());

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

    if !command.playlists.is_empty() {
        let mut playlist_query = playlists::table.into_boxed().select(playlists::id);
        for playlist in command.playlists {
            playlist_query = playlist_query.filter(playlists::name.like(playlist));
        }
        let playlistids = playlist_query
            .load::<i32>(connection)
            .expect("Error fetching playlists");
        let songids = playlistref::table
            .into_boxed()
            .select(playlistref::song)
            .filter(playlistref::playlist.eq_any(playlistids))
            .load::<i32>(connection)
            .expect("Error fetching playlist ref ids");
        query = query.filter(songs::id.eq_any(songids));
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

    let results = query.load::<Song>(connection).expect("Error loading songs");
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
