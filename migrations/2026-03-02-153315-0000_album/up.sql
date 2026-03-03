CREATE TABLE albums (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  image INTEGER NULL,
  artist INTEGER NOT NULL,
  last_updated INTEGER NOT NULL,
  FOREIGN KEY (image) REFERENCES images(id),
  FOREIGN KEY (artist) REFERENCES artists(id),
  CONSTRAINT UQ_artist_album UNIQUE (artist, name)
)