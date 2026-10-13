BEGIN;

CREATE TABLE IF NOT EXISTS color (
     color_id INTEGER PRIMARY KEY,
     color_name TEXT NOT NULL,
     color_description TEXT NOT NULL,
     red INTEGER,
     green INTEGER,
     blue INTEGER,
     alpha INTEGER
);

CREATE TABLE IF NOT EXISTS databases(
    database_id INTEGER PRIMARY KEY,
    database_name TEXT NOT NULL,
    database_description TEXT NOT NULL,
    location TEXT NOT NULL,
    is_in_cloud INTEGER NOT NULL CHECK (is_in_cloud IN (0, 1)),
    is_ssh INTEGER NOT NULL CHECK (is_ssh IN (0, 1)),
    username TEXT,
    server_address TEXT,
    location_private_key TEXT,
    password_encrypted TEXT,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (color_id) REFERENCES color(color_id)
);

COMMIT;