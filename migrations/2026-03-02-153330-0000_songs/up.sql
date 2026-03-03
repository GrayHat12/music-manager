CREATE TABLE songs (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  genre INTEGER NULL,
  artist INTEGER NULL,
  album INTEGER NULL,
  cover INTEGER NULL,
  title VARCHAR NOT NULL,
  release INTEGER NULL,
  trackno INTEGER NULL,
  metatags TEXT NOT NULL,
  buffer BLOB NOT NULL UNIQUE,
  last_updated INTEGER NOT NULL,
  FOREIGN KEY(genre) REFERENCES genre(id),
  FOREIGN KEY(artist) REFERENCES artists(id),
  FOREIGN KEY(album) REFERENCES albums(id),
  FOREIGN KEY(cover) REFERENCES images(id)
)