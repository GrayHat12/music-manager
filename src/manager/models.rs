use std::{collections::HashMap, usize};

use diesel::prelude::*;
use symphonia::core::meta::StandardTagKey;

use crate::manager::schema;
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
    pub buffer: Vec<u8>,
    pub last_updated: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SongLite {
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
    pub fn from_song_lite(lite: &SongLite, connection: &mut SqliteConnection) -> Self {
        songs::table
            .filter(songs::id.eq(lite.id))
            .first::<Song>(connection)
            .expect("Failed fetching song")
    }
    pub fn get_metatags(self) -> HashMap<StandardTagKey, String> {
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

    pub fn get_artist(self, connection: &mut SqliteConnection) -> Option<Artist> {
        if let Some(id) = self.artist {
            Some(
                artists::table
                    .filter(artists::id.eq(id))
                    .first::<Artist>(connection)
                    .expect("Error fetching artist"),
            )
        } else {
            None
        }
    }

    pub fn get_album(self, connection: &mut SqliteConnection) -> Option<Album> {
        if let Some(id) = self.album {
            Some(
                albums::table
                    .filter(albums::id.eq(id))
                    .first::<Album>(connection)
                    .expect("Error fetching album"),
            )
        } else {
            None
        }
    }

    pub fn get_genre(self, connection: &mut SqliteConnection) -> Option<Genre> {
        if let Some(id) = self.genre {
            Some(
                genre::table
                    .filter(genre::id.eq(id))
                    .first::<Genre>(connection)
                    .expect("Error fetching genre"),
            )
        } else {
            None
        }
    }

    pub fn get_features(self, connection: &mut SqliteConnection) -> Vec<Artist> {
        artists::table
            .filter(
                artists::id.eq_any(
                    features::table
                        .select(features::artist)
                        .filter(features::song.eq(self.id))
                        .load::<i32>(connection)
                        .expect("failed fetching features"),
                ),
            )
            .load::<Artist>(connection)
            .expect("Error failed fetching artists")
    }

    pub fn get_cover(self, connection: &mut SqliteConnection) -> Option<CoverImage> {
        if let Some(id) = self.cover {
            Some(
                images::table
                    .filter(images::id.eq(id))
                    .first::<CoverImage>(connection)
                    .expect("Error fetching cover"),
            )
        } else {
            None
        }
    }
}

impl SongLite {
    pub fn get_metatags(self) -> HashMap<StandardTagKey, String> {
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

    pub fn get_artist(self, connection: &mut SqliteConnection) -> Option<Artist> {
        if let Some(id) = self.artist {
            Some(
                artists::table
                    .filter(artists::id.eq(id))
                    .first::<Artist>(connection)
                    .expect("Error fetching artist"),
            )
        } else {
            None
        }
    }

    pub fn get_album(self, connection: &mut SqliteConnection) -> Option<Album> {
        if let Some(id) = self.album {
            Some(
                albums::table
                    .filter(albums::id.eq(id))
                    .first::<Album>(connection)
                    .expect("Error fetching album"),
            )
        } else {
            None
        }
    }

    pub fn get_genre(self, connection: &mut SqliteConnection) -> Option<Genre> {
        if let Some(id) = self.genre {
            Some(
                genre::table
                    .filter(genre::id.eq(id))
                    .first::<Genre>(connection)
                    .expect("Error fetching genre"),
            )
        } else {
            None
        }
    }

    pub fn get_features(self, connection: &mut SqliteConnection) -> Vec<Artist> {
        artists::table
            .filter(
                artists::id.eq_any(
                    features::table
                        .select(features::artist)
                        .filter(features::song.eq(self.id))
                        .load::<i32>(connection)
                        .expect("failed fetching features"),
                ),
            )
            .load::<Artist>(connection)
            .expect("Error failed fetching artists")
    }

    pub fn get_cover(self, connection: &mut SqliteConnection) -> Option<CoverImage> {
        if let Some(id) = self.cover {
            Some(
                images::table
                    .filter(images::id.eq(id))
                    .first::<CoverImage>(connection)
                    .expect("Error fetching cover"),
            )
        } else {
            None
        }
    }
}

impl Album {
    pub fn get_cover(self, connection: &mut SqliteConnection) -> Option<CoverImage> {
        if let Some(id) = self.image {
            Some(
                images::table
                    .filter(images::id.eq(id))
                    .first::<CoverImage>(connection)
                    .expect("Error fetching cover"),
            )
        } else {
            None
        }
    }
}

impl Artist {
    pub fn get_cover(self, connection: &mut SqliteConnection) -> Option<CoverImage> {
        if let Some(id) = self.image {
            Some(
                images::table
                    .filter(images::id.eq(id))
                    .first::<CoverImage>(connection)
                    .expect("Error fetching cover"),
            )
        } else {
            None
        }
    }
}
