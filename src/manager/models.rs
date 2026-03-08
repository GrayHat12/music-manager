use std::collections::HashMap;
use std::usize;

use diesel::prelude::*;
use diesel::result::Error;
use symphonia::core::meta::StandardTagKey;

use crate::manager::schema::{self, playlistref, playlists};
use crate::manager::schema::{albums, artists, features, genre, images, songs};

pub const TAGS: [StandardTagKey; 111] = [
    StandardTagKey::AcoustidFingerprint,
    StandardTagKey::AcoustidId,
    StandardTagKey::Album,
    StandardTagKey::AlbumArtist,
    StandardTagKey::Arranger,
    StandardTagKey::Artist,
    StandardTagKey::Bpm,
    StandardTagKey::Comment,
    StandardTagKey::Compilation,
    StandardTagKey::Composer,
    StandardTagKey::Conductor,
    StandardTagKey::ContentGroup,
    StandardTagKey::Copyright,
    StandardTagKey::Date,
    StandardTagKey::Description,
    StandardTagKey::DiscNumber,
    StandardTagKey::DiscSubtitle,
    StandardTagKey::DiscTotal,
    StandardTagKey::EncodedBy,
    StandardTagKey::Encoder,
    StandardTagKey::EncoderSettings,
    StandardTagKey::EncodingDate,
    StandardTagKey::Engineer,
    StandardTagKey::Ensemble,
    StandardTagKey::Genre,
    StandardTagKey::IdentAsin,
    StandardTagKey::IdentBarcode,
    StandardTagKey::IdentCatalogNumber,
    StandardTagKey::IdentEanUpn,
    StandardTagKey::IdentIsrc,
    StandardTagKey::IdentPn,
    StandardTagKey::IdentPodcast,
    StandardTagKey::IdentUpc,
    StandardTagKey::Label,
    StandardTagKey::Language,
    StandardTagKey::License,
    StandardTagKey::Lyricist,
    StandardTagKey::Lyrics,
    StandardTagKey::MediaFormat,
    StandardTagKey::MixDj,
    StandardTagKey::MixEngineer,
    StandardTagKey::Mood,
    StandardTagKey::MovementName,
    StandardTagKey::MovementNumber,
    StandardTagKey::MusicBrainzAlbumArtistId,
    StandardTagKey::MusicBrainzAlbumId,
    StandardTagKey::MusicBrainzArtistId,
    StandardTagKey::MusicBrainzDiscId,
    StandardTagKey::MusicBrainzGenreId,
    StandardTagKey::MusicBrainzLabelId,
    StandardTagKey::MusicBrainzOriginalAlbumId,
    StandardTagKey::MusicBrainzOriginalArtistId,
    StandardTagKey::MusicBrainzRecordingId,
    StandardTagKey::MusicBrainzReleaseGroupId,
    StandardTagKey::MusicBrainzReleaseStatus,
    StandardTagKey::MusicBrainzReleaseTrackId,
    StandardTagKey::MusicBrainzReleaseType,
    StandardTagKey::MusicBrainzTrackId,
    StandardTagKey::MusicBrainzWorkId,
    StandardTagKey::Opus,
    StandardTagKey::OriginalAlbum,
    StandardTagKey::OriginalArtist,
    StandardTagKey::OriginalDate,
    StandardTagKey::OriginalFile,
    StandardTagKey::OriginalWriter,
    StandardTagKey::Owner,
    StandardTagKey::Part,
    StandardTagKey::PartTotal,
    StandardTagKey::Performer,
    StandardTagKey::Podcast,
    StandardTagKey::PodcastCategory,
    StandardTagKey::PodcastDescription,
    StandardTagKey::PodcastKeywords,
    StandardTagKey::Producer,
    StandardTagKey::PurchaseDate,
    StandardTagKey::Rating,
    StandardTagKey::ReleaseCountry,
    StandardTagKey::ReleaseDate,
    StandardTagKey::Remixer,
    StandardTagKey::ReplayGainAlbumGain,
    StandardTagKey::ReplayGainAlbumPeak,
    StandardTagKey::ReplayGainTrackGain,
    StandardTagKey::ReplayGainTrackPeak,
    StandardTagKey::Script,
    StandardTagKey::SortAlbum,
    StandardTagKey::SortAlbumArtist,
    StandardTagKey::SortArtist,
    StandardTagKey::SortComposer,
    StandardTagKey::SortTrackTitle,
    StandardTagKey::TaggingDate,
    StandardTagKey::TrackNumber,
    StandardTagKey::TrackSubtitle,
    StandardTagKey::TrackTitle,
    StandardTagKey::TrackTotal,
    StandardTagKey::TvEpisode,
    StandardTagKey::TvEpisodeTitle,
    StandardTagKey::TvNetwork,
    StandardTagKey::TvSeason,
    StandardTagKey::TvShowTitle,
    StandardTagKey::Url,
    StandardTagKey::UrlArtist,
    StandardTagKey::UrlCopyright,
    StandardTagKey::UrlInternetRadio,
    StandardTagKey::UrlLabel,
    StandardTagKey::UrlOfficial,
    StandardTagKey::UrlPayment,
    StandardTagKey::UrlPodcast,
    StandardTagKey::UrlPurchase,
    StandardTagKey::UrlSource,
    StandardTagKey::Version,
    StandardTagKey::Writer,
];

pub fn get_now() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::images)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Artist))]
#[diesel(belongs_to(Song))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CoverImage {
    pub id: i32,
    pub buffer: Vec<u8>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Album))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub image: Option<i32>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub image: Option<i32>,
    pub artist: i32,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::genre)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Genre {
    pub id: i32,
    pub last_updated: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub image: Option<i32>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::playlistref)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlaylistRef {
    pub id: i32,
    pub song: i32,
    pub playlist: i32,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Song {
    pub id: i32,
    pub genre: Option<i32>,
    pub artist: Option<i32>,
    pub album: Option<i32>,
    pub cover: Option<i32>,
    pub title: String,
    pub release: Option<i32>,
    pub trackno: Option<i32>,
    metatags: String,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub buffer: Vec<u8>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = artists)]
pub struct NewArtist {
    pub name: String,
    pub image: Option<i32>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbum {
    pub name: String,
    pub artist: i32,
    pub image: Option<i32>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = genre)]
pub struct NewGenre {
    pub name: String,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = features)]
pub struct NewFeature {
    pub artist: i32,
    pub song: i32,
    pub last_updated: i32,
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong {
    pub genre: Option<i32>,
    pub artist: Option<i32>,
    pub album: Option<i32>,
    pub cover: Option<i32>,
    pub title: String,
    pub release: Option<i32>,
    pub trackno: Option<i32>,
    pub metatags: String,
    pub buffer: Vec<u8>,
    pub last_updated: i32,
}

#[derive(Insertable)]
#[diesel(table_name = playlists)]
pub struct NewPlaylist {
    pub name: String,
    pub image: Option<i32>,
    pub last_updated: i32,
}

#[derive(Insertable)]
#[diesel(table_name = playlistref)]
pub struct SongToPlaylist {
    pub song: i32,
    pub playlist: i32,
    pub last_updated: i32,
}

pub trait ToEnum<T> {
    fn to_enum(value: T) -> Option<Self>
    where
        Self: Sized;
    fn from_enum(self) -> Option<T>;
}

impl ToEnum<usize> for StandardTagKey {
    fn to_enum(value: usize) -> Option<Self>
    where
        Self: Sized,
    {
        TAGS.get(value).copied()
    }

    fn from_enum(self) -> Option<usize> {
        for (idx, item) in TAGS.iter().enumerate() {
            if self == *item {
                return Some(idx);
            }
        }
        return None;
    }
}

impl Song {
    pub fn new(connection: &mut SqliteConnection, song: &NewSong) -> Self {
        diesel::insert_into(songs::table)
            .values((
                songs::genre.eq(&song.genre),
                songs::artist.eq(&song.artist),
                songs::album.eq(&song.album),
                songs::cover.eq(&song.cover),
                songs::title.eq(&song.title),
                songs::release.eq(&song.release),
                songs::trackno.eq(&song.trackno),
                songs::metatags.eq(&song.metatags),
                songs::buffer.eq(&song.buffer),
                songs::last_updated.eq(&song.last_updated),
            ))
            .on_conflict(songs::buffer)
            .do_nothing()
            .returning(Song::as_returning())
            .get_result(connection)
            .expect("failed song insert")
    }

    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        songs::table
            .select(Song::as_select())
            .filter(songs::id.eq(&id))
            .first(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(songs::table.filter(songs::id.eq(id))).execute(connection)
    }

    // features
    pub fn add_features(
        &self,
        connection: &mut SqliteConnection,
        artists: &Vec<i32>,
    ) -> Result<Vec<Artist>, Error> {
        // validate artist ids, not efficient but it's fine for now
        for artist in artists {
            _ = Artist::from_id(connection, *artist)?;
        }
        for artist in artists {
            diesel::insert_into(features::table)
                .values(NewFeature {
                    artist: *artist,
                    song: self.id,
                    last_updated: get_now(),
                })
                .on_conflict((features::artist, features::song))
                .do_nothing()
                .execute(connection)?;
        }
        Ok(self.get_features(connection))
    }

    pub fn remove_features(
        &self,
        connection: &mut SqliteConnection,
        artists: &Vec<i32>,
    ) -> Vec<Artist> {
        diesel::delete(features::table)
            .filter(
                features::song
                    .eq(self.id)
                    .and(features::artist.eq_any(artists)),
            )
            .execute(connection)
            .expect("failed feature insert");

        self.get_features(connection)
    }

    pub fn get_features(&self, connection: &mut SqliteConnection) -> Vec<Artist> {
        artists::table
            .inner_join(features::table)
            .filter(features::song.eq(self.id))
            .select(Artist::as_select())
            .load::<Artist>(connection)
            .expect("failed fetching artists")
    }

    // genre
    pub fn get_genre(&self, connection: &mut SqliteConnection) -> Option<Genre> {
        match self.genre {
            Some(id) => Some(Genre::from_id(connection, id).expect("failed fetching genre")),
            None => None,
        }
    }
    pub fn set_genre(&mut self, connection: &mut SqliteConnection, name: &String) {
        self.genre = match Genre::from_name(connection, name) {
            Ok(genre) => {
                diesel::update(songs::table.filter(songs::id.eq(self.id)))
                    .set(songs::genre.eq(genre.id))
                    .execute(connection)
                    .expect("failed updating song");
                Some(genre.id)
            }
            Err(_) => self.genre,
        };
    }

    // artist
    pub fn get_artist(&self, connection: &mut SqliteConnection) -> Option<Artist> {
        match self.artist {
            Some(id) => Some(Artist::from_id(connection, id).expect("failed fetching artist")),
            None => None,
        }
    }
    pub fn set_artist(&mut self, connection: &mut SqliteConnection, name: &String) {
        self.artist = match Artist::from_name(connection, name) {
            Ok(artist) => {
                diesel::update(songs::table.filter(songs::id.eq(self.id)))
                    .set(songs::artist.eq(artist.id))
                    .execute(connection)
                    .expect("failed updating song");
                Some(artist.id)
            }
            Err(_) => self.artist,
        };
    }

    // album
    pub fn get_album(&self, connection: &mut SqliteConnection) -> Option<Album> {
        match self.album {
            Some(id) => Some(Album::from_id(connection, id).expect("failed fetching album")),
            None => None,
        }
    }
    pub fn set_album(&mut self, connection: &mut SqliteConnection, id: i32) {
        self.album = match Album::from_id(connection, id) {
            Ok(album) => {
                diesel::update(songs::table.filter(songs::id.eq(self.id)))
                    .set(songs::album.eq(album.id))
                    .execute(connection)
                    .expect("failed updating song");
                Some(album.id)
            }
            Err(_) => self.album,
        };
    }

    // cover
    pub fn get_cover(&self, connection: &mut SqliteConnection) -> Option<CoverImage> {
        match self.cover {
            Some(id) => Some(CoverImage::from_id(connection, id).expect("failed fetching art")),
            None => None,
        }
    }
    pub fn set_cover(&mut self, connection: &mut SqliteConnection, buffer: &Vec<u8>) {
        self.cover = Some(CoverImage::new(connection, buffer).id);
    }

    // buffer
    pub fn get_buffer(&self, connection: &mut SqliteConnection) -> Vec<u8> {
        songs::table
            .select(songs::buffer)
            .filter(songs::id.eq(self.id))
            .first(connection)
            .expect("failed loading song buffer")
    }

    // playlist
    pub fn add_to_playlist(
        &self,
        connection: &mut SqliteConnection,
        playlist: i32,
    ) -> Result<&Self, Error> {
        match Playlist::from_id(connection, playlist) {
            Ok(playlist) => match playlist.add_song(connection, self.id) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // meta tags
    pub fn get_metatags(&self) -> HashMap<StandardTagKey, String> {
        let mut tags = HashMap::<StandardTagKey, String>::new();
        let parsed_list: Result<Vec<String>, serde_json::Error> =
            serde_json::from_str(&self.metatags);

        match parsed_list {
            Ok(list) => {
                for (index, value) in list.iter().enumerate() {
                    match StandardTagKey::to_enum(index) {
                        Some(enm) => match tags.insert(enm, value.clone()) {
                            Some(_) => {}
                            None => {}
                        },
                        None => {}
                    };
                }
            }
            Err(_) => {}
        };
        tags
    }
}

impl Album {
    pub fn new(connection: &mut SqliteConnection, new_album: NewAlbum) -> Self {
        diesel::insert_into(albums::table)
            .values((
                albums::name.eq(&new_album.name),
                albums::image.eq(&new_album.image),
                albums::artist.eq(&new_album.artist),
                albums::last_updated.eq(new_album.last_updated),
            ))
            .on_conflict((albums::name, albums::artist))
            .do_nothing()
            .returning(Album::as_returning())
            .get_result(connection)
            .expect("failed album insert")
    }

    // only fetch
    pub fn from_name(
        connection: &mut SqliteConnection,
        album: &String,
        artist: &String,
    ) -> Result<Self, Error> {
        match Artist::from_name(connection, artist) {
            Ok(artist) => albums::table
                .filter(albums::name.eq(&album).and(albums::artist.eq(artist.id)))
                .first(connection),
            Err(e) => Err(e),
        }
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(albums::table.filter(albums::id.eq(id))).execute(connection)
    }

    // only fetch
    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        albums::table.filter(albums::id.eq(&id)).first(connection)
    }

    pub fn update_name(
        &mut self,
        connection: &mut SqliteConnection,
        name: String,
    ) -> Result<&mut Self, Error> {
        match diesel::update(albums::table.filter(albums::id.eq(self.id)))
            .set((albums::name.eq(&name), albums::last_updated.eq(get_now())))
            .returning(Album::as_returning())
            .get_result::<Album>(connection)
        {
            Ok(a) => {
                self.name = a.name;
                self.last_updated = a.last_updated;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn update_image(
        &mut self,
        connection: &mut SqliteConnection,
        image: Option<i32>,
    ) -> Result<&mut Self, Error> {
        match diesel::update(albums::table.filter(albums::id.eq(self.id)))
            .set((albums::image.eq(image), albums::last_updated.eq(get_now())))
            .returning(Album::as_returning())
            .get_result::<Album>(connection)
        {
            Ok(n) => {
                self.name = n.name;
                self.image = n.image;
                self.last_updated = n.last_updated;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }
}

impl Artist {
    pub fn new(connection: &mut SqliteConnection, new_artist: NewArtist) -> Self {
        diesel::insert_into(artists::table)
            .values((
                artists::name.eq(&new_artist.name),
                artists::image.eq(&new_artist.image),
                artists::last_updated.eq(new_artist.last_updated),
            ))
            .on_conflict(artists::name)
            .do_nothing()
            .returning(Artist::as_returning())
            .get_result(connection)
            .expect("failed artist insert")
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(artists::table.filter(artists::id.eq(id))).execute(connection)
    }

    // only fetch
    pub fn from_name(connection: &mut SqliteConnection, name: &String) -> Result<Self, Error> {
        artists::table
            .filter(artists::name.eq(&name))
            .first(connection)
    }

    // only fetch
    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        artists::table.filter(artists::id.eq(&id)).first(connection)
    }

    pub fn update_name(
        &mut self,
        connection: &mut SqliteConnection,
        name: String,
    ) -> Result<&mut Self, Error> {
        match diesel::update(artists::table.filter(artists::id.eq(self.id)))
            .set((artists::name.eq(&name), artists::last_updated.eq(get_now())))
            .returning(Artist::as_returning())
            .get_result::<Artist>(connection)
        {
            Ok(n) => {
                self.name = n.name;
                self.image = n.image;
                self.last_updated = n.last_updated;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn update_image(
        &mut self,
        connection: &mut SqliteConnection,
        image: Option<i32>,
    ) -> Result<&mut Self, Error> {
        match diesel::update(artists::table.filter(artists::id.eq(self.id)))
            .set((
                artists::image.eq(image),
                artists::last_updated.eq(get_now()),
            ))
            .returning(Artist::as_returning())
            .get_result::<Artist>(connection)
        {
            Ok(n) => {
                self.name = n.name;
                self.image = n.image;
                self.last_updated = n.last_updated;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }
}

impl Genre {
    // create if not exists and return genre object
    pub fn new(connection: &mut SqliteConnection, name: &String) -> Self {
        diesel::insert_into(genre::table)
            .values((genre::name.eq(name), genre::last_updated.eq(get_now())))
            .on_conflict(genre::name)
            .do_nothing()
            .returning(Genre::as_returning())
            .get_result(connection)
            .expect("failed genre insert")
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(genre::table.filter(genre::id.eq(id))).execute(connection)
    }

    // only fetch
    pub fn from_name(connection: &mut SqliteConnection, name: &String) -> Result<Self, Error> {
        genre::table.filter(genre::name.eq(&name)).first(connection)
    }

    // only fetch
    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        genre::table.filter(genre::id.eq(&id)).first(connection)
    }

    pub fn update_name(
        &mut self,
        connection: &mut SqliteConnection,
        name: String,
    ) -> Result<&mut Self, Error> {
        match diesel::update(genre::table.filter(genre::id.eq(self.id)))
            .set((genre::name.eq(&name), genre::last_updated.eq(get_now())))
            .execute(connection)
        {
            Ok(_) => match Self::from_id(connection, self.id) {
                Ok(n) => {
                    self.name = n.name;
                    self.last_updated = n.last_updated;
                    Ok(self)
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl CoverImage {
    // create if not exists and return genre object
    pub fn new(connection: &mut SqliteConnection, image: &Vec<u8>) -> Self {
        diesel::insert_into(images::table)
            .values((
                images::buffer.eq(&image),
                images::last_updated.eq(get_now()),
            ))
            .on_conflict(images::buffer)
            .do_nothing()
            .returning(CoverImage::as_returning())
            .get_result(connection)
            .expect("failed image insert")
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(images::table.filter(images::id.eq(id))).execute(connection)
    }

    // only fetch
    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        images::table.filter(images::id.eq(&id)).first(connection)
    }

    pub fn update_image(
        &mut self,
        connection: &mut SqliteConnection,
        image: &Vec<u8>,
    ) -> Result<&mut Self, Error> {
        match diesel::update(images::table.filter(images::id.eq(self.id)))
            .set((
                images::buffer.eq(&image),
                images::last_updated.eq(get_now()),
            ))
            .returning(CoverImage::as_returning())
            .get_result::<CoverImage>(connection)
        {
            Ok(cover) => {
                self.buffer = cover.buffer;
                self.last_updated = cover.last_updated;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }
}

impl Playlist {
    pub fn new(connection: &mut SqliteConnection, new_playlist: &NewPlaylist) -> Self {
        diesel::insert_into(playlists::table)
            .values(new_playlist)
            .on_conflict_do_nothing()
            .returning(Playlist::as_returning())
            .get_result(connection)
            .expect("failed inserting to playlist")
    }

    pub fn from_id(connection: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        playlists::table
            .select(Playlist::as_select())
            .filter(playlists::id.eq(&id))
            .first(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize, Error> {
        diesel::delete(playlists::table.filter(playlists::id.eq(id))).execute(connection)
    }

    pub fn add_song(&self, connection: &mut SqliteConnection, song: i32) -> Result<&Self, Error> {
        diesel::insert_into(playlistref::table)
            .values(&SongToPlaylist {
                song,
                playlist: self.id,
                last_updated: get_now(),
            })
            .on_conflict_do_nothing()
            .execute(connection)?;
        Ok(self)
    }

    pub fn remove_song(
        &self,
        connection: &mut SqliteConnection,
        song: i32,
    ) -> Result<&Self, Error> {
        diesel::delete(playlistref::table)
            .filter(
                playlistref::song
                    .eq(song)
                    .and(playlistref::playlist.eq(self.id)),
            )
            .execute(connection)?;
        Ok(self)
    }
}
