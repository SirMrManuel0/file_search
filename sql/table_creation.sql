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

CREATE TABLE IF NOT EXISTS item (
    item_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    description TEXT NOT NULL,
    color_id INTEGER NOT NULL,
    is_directory INTEGER NOT NULL CHECK (is_directory IN (0, 1)),
    FOREIGN KEY (color_id) REFERENCES color(color_id)
);

CREATE TABLE IF NOT EXISTS tag (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE,
    tag_description TEXT NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (color_id) REFERENCES color(color_id)
);

CREATE TABLE IF NOT EXISTS category (
    category_id INTEGER PRIMARY KEY,
    category_name TEXT NOT NULL UNIQUE,
    category_description TEXT NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (color_id) REFERENCES color(color_id)
);

CREATE TABLE IF NOT EXISTS item_tag_index (
    item_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (item_id, tag_id),
    FOREIGN KEY (item_id) REFERENCES item(item_id),
    FOREIGN KEY (tag_id) REFERENCES tag(tag_id)
);

CREATE TABLE IF NOT EXISTS item_category_index (
    item_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (item_id, category_id),
    FOREIGN KEY (item_id) REFERENCES item(item_id),
    FOREIGN KEY (category_id) REFERENCES category(category_id)
);

CREATE TABLE IF NOT EXISTS tag_linked_tag_index (
    tag_id INTEGER,
    linked_tag_id INTEGER,
    PRIMARY KEY (tag_id, linked_tag_id),
    FOREIGN KEY (tag_id) REFERENCES tag(tag_id),
    FOREIGN KEY (linked_tag_id) REFERENCES tag(tag_id)
);

CREATE TABLE IF NOT EXISTS tag_category_index (
    tag_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (tag_id, category_id),
    FOREIGN KEY (tag_id) REFERENCES tag(tag_id),
    FOREIGN KEY (category_id) REFERENCES category(category_id)
);

COMMIT;