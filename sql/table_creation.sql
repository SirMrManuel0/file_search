BEGIN;

CREATE TABLE IF NOT EXISTS item (
    item_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    red INTEGER NOT NULL,
    green INTEGER NOT NULL,
    blue INTEGER NOT NULL,
    is_directory INTEGER NOT NULL CHECK (is_directory IN (0, 1))
);

CREATE TABLE IF NOT EXISTS tag (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE,
    red INTEGER NOT NULL,
    green INTEGER NOT NULL,
    blue INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS category (
    category_id INTEGER PRIMARY KEY,
    category_name TEXT NOT NULL UNIQUE,
    red INTEGER NOT NULL,
    green INTEGER NOT NULL,
    blue INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS color (
    color_id INTEGER PRIMARY KEY,
    red INTEGER,
    green INTEGER,
    blue INTEGER,
    alpha INTEGER
);

CREATE TABLE IF NOT EXISTS item_tag_index (
    item_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (item_id, tag_id)
);

CREATE TABLE IF NOT EXISTS item_category_index (
    item_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (item_id, category_id)
);

CREATE TABLE IF NOT EXISTS tag_linked_tag_index (
    tag_id INTEGER,
    linked_tag_id INTEGER,
    PRIMARY KEY (tag_id, linked_tag_id)
);

CREATE TABLE IF NOT EXISTS tag_category_index (
    tag_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (tag_id, category_id)
);

COMMIT;