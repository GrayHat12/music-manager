// mod bfort;
// pub mod data;
// mod implementations;
// mod models;
// mod traits;

use std::{
    collections::HashMap,
    fmt::Error,
    io::{Read, Seek},
    path::Path,
};

use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey},
    probe::Hint,
};

#[derive(Debug)]
pub struct LoadedSong {
    pub tags: HashMap<StandardTagKey, String>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub release: Option<String>,
    pub trackno: Option<i32>,
    pub buffer: Vec<u8>,
    pub art: Option<String>,
    pub features: Vec<String>,
}

pub fn load_song(path_str: String) -> Result<LoadedSong, Error> {
    let path = Path::new(path_str.as_str());
    let mut src = std::fs::File::open(path).expect("failed to open media");
    let mut buffer = Vec::new();
    src.read_to_end(&mut buffer).expect("Failed reading file");
    src.rewind().expect("failed rewind on file");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let mut format = probed.format;

    let mut song = LoadedSong {
        tags: HashMap::new(),
        title: match path.file_prefix() {
            Some(v) => match v.to_str() {
                Some(n) => n.to_string(),
                None => "<unknown>".to_string(),
            },
            None => "<unknown>".to_string(),
        },
        artist: None,
        album: None,
        genre: None,
        release: None,
        trackno: None,
        buffer,
        art: None,
        features: Vec::new(),
    };

    let probed_metadata = probed.metadata.get();
    let format_metadata = format.metadata();
    match probed_metadata.or(Some(format_metadata)) {
        Some(mut meta) => match meta.skip_to_latest() {
            Some(meta) => {
                // println!("total tags {:?}", meta.tags().len());
                for tag in meta.tags() {
                    if let Some(std_key) = tag.std_key {
                        // println!("got key {:?}", std_key);
                        song.tags.insert(std_key, tag.value.to_string());
                        match std_key {
                            StandardTagKey::TrackTitle => {
                                // println!("got title {:#?}", tag.value.to_string());
                                song.title = tag.value.to_string();
                            }
                            StandardTagKey::Artist => {
                                // println!("got artist {:#?}", tag.value.to_string());
                                let artist = tag.value.to_string();
                                for (index, name) in artist
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .enumerate()
                                {
                                    if index == 0 {
                                        song.artist = Some(name.to_string());
                                    } else {
                                        song.features.push(name.to_string());
                                    }
                                }
                            }
                            StandardTagKey::Album => {
                                // println!("got album {:#?}", tag.value.to_string());
                                song.album = Some(tag.value.to_string());
                            }
                            StandardTagKey::Genre => {
                                // println!("got genre {:#?}", tag.value.to_string());
                                song.genre = Some(tag.value.to_string());
                            }
                            StandardTagKey::Date => {
                                // println!(
                                //     "got date {:#?}",
                                //     tag.value.to_string().parse::<u32>().ok()
                                // );
                                song.release = Some(tag.value.to_string());
                            }
                            StandardTagKey::TrackNumber => {
                                // println!(
                                //     "got track number {:#?}",
                                //     tag.value.to_string().parse::<u32>().ok()
                                // );
                                song.trackno = tag.value.to_string().parse::<i32>().ok();
                            }
                            _ => {
                                // println!("got key={:#?} value={:#?}", other, tag.value.to_string());
                            }
                        }
                    }
                }
            }
            None => println!("inner none"),
        },
        None => println!("outer none"),
    };
    // println!("returning song");
    Ok(song)
}
