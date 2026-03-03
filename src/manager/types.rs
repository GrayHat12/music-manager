use clap::Args;

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Init {}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Add {
    /// Path of the song to add
    #[arg(short, long)]
    pub path: String,

    /// Genre of the song to add
    #[arg(short, long)]
    pub genre: Option<String>,

    /// Artist of the song to add
    #[arg(short, long)]
    pub artist: Option<String>,

    /// Features on the song
    #[arg(short, long, value_delimiter = ',')]
    pub features: Vec<String>,

    /// Album of the song to add
    #[arg(long)]
    pub album: Option<String>,

    /// Image path for the song to add
    #[arg(long)]
    pub art: Option<String>,

    /// Release year of the song to add
    #[arg(short, long)]
    pub release: Option<u32>,

    /// Title of the song to add
    #[arg(short, long)]
    pub title: Option<String>,

    /// Index of the song to add
    #[arg(short, long)]
    pub index: Option<u8>,
}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Remove {
    /// Song id to remove
    #[arg(short, long)]
    pub id: String,
}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct List {
    /// Genre filter
    #[arg(short, long, value_delimiter = ',')]
    pub genre: Vec<String>,

    /// Artist filter
    #[arg(short, long, value_delimiter = ',')]
    pub artist: Vec<String>,

    /// Features filter
    #[arg(short, long, value_delimiter = ',')]
    pub features: Vec<String>,

    /// Album filter
    #[arg(long, value_delimiter = ',')]
    pub album: Vec<String>,

    /// Release year filter
    #[arg(short, long, value_delimiter = ',')]
    pub release: Vec<u32>,

    /// Title filter
    #[arg(short, long, value_delimiter = ',')]
    pub title: Vec<String>,

    /// Index filter
    #[arg(short, long, value_delimiter = ',')]
    pub index: Vec<u8>,

    /// only shows these fields
    #[arg(short, long, value_delimiter = ',', default_values=&["id", "genre", "artist", "features", "album", "index", "title", "release"])]
    pub only_show: Option<Vec<String>>,
}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListArtists {
    /// Genre filter
    #[arg(short, long, value_delimiter = ',')]
    pub genre: Vec<String>,

    /// Album filter
    #[arg(long, value_delimiter = ',')]
    pub album: Vec<String>,

    /// Release year filter
    #[arg(short, long, value_delimiter = ',')]
    pub release: Vec<u32>,
}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListGenres {
    /// Album filter
    #[arg(long, value_delimiter = ',')]
    pub album: Vec<String>,

    /// Release year filter
    #[arg(short, long, value_delimiter = ',')]
    pub release: Vec<u32>,
}

#[derive(Args, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListAlbums {
    /// Genre filter
    #[arg(short, long, value_delimiter = ',')]
    pub genre: Vec<String>,

    /// Artist filter
    #[arg(short, long, value_delimiter = ',')]
    pub artist: Vec<String>,

    /// Release year filter
    #[arg(short, long, value_delimiter = ',')]
    pub release: Vec<u32>,
}
