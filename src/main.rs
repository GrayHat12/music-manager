use std::io;

pub mod commands;
pub mod library;
pub mod types;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use music_manager::establish_connection;

use crate::commands::{add_song, list_albums, list_artists, list_genres, list_songs, remove_song};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Parser)]
#[command(name = "Music Manager")]
#[command(about = "A music library manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Library path
    #[arg(short, long, default_value = "~/music.sql")]
    root_path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a new music library
    Init(types::Init),

    /// Add song to library
    Add(types::Add),

    /// Remove song from library
    Remove(types::Remove),

    /// List songs
    List(types::List),

    /// List artists
    ListArtists(types::ListArtists),

    /// List genres
    ListGenres(types::ListGenres),

    /// List albums
    ListAlbums(types::ListAlbums),

    /// Generate shell completions (you need to add it to your shell launch `.zshrc` / `.bashrc` according to your shell)
    #[command(hide = true)]
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    // Use the CLI path if provided, otherwise fallback to env
    let db_url = if std::path::Path::new(&cli.root_path).exists() || cli.root_path != "~/music.sql"
    {
        cli.root_path.clone()
    } else {
        dotenvy::var("DATABASE_URL").unwrap_or(cli.root_path)
    };

    let mut connection = establish_connection(db_url);

    match cli.command {
        Commands::Init(_) => {
            // println!("got init {:#?}", init);
            connection.run_pending_migrations(MIGRATIONS).unwrap();
            println!("Database tables initialized successfully.");
        }
        Commands::Add(add) => {
            // println!("got add {:#?}", add);
            add_song(&mut connection, add);
        }
        Commands::Remove(remove) => {
            // println!("got remove {:#?}", remove);
            remove_song(&mut connection, remove);
        }
        Commands::List(list) => {
            // println!("got list {:#?}", list);
            list_songs(&mut connection, list);
        }
        Commands::ListArtists(list) => {
            // println!("got list_artists {:#?}", list);
            list_artists(&mut connection, list);
        }
        Commands::ListGenres(list) => {
            // println!("got list_genres {:#?}", list);
            list_genres(&mut connection, list);
        }
        Commands::ListAlbums(list) => {
            // println!("got list_albums {:#?}", list);
            list_albums(&mut connection, list);
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command(); // This builds the clap Command
            let bin_name = cmd.get_name().to_string();

            // This prints the shell script to stdout
            generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }
}
