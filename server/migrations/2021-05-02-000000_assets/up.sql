CREATE TABLE IF NOT EXISTS assets (
  id VARCHAR NOT NULL PRIMARY KEY,
  symbol VARCHAR,
  name VARCHAR
);

CREATE TABLE IF NOT EXISTS asset_categories (
  id VARCHAR NOT NULL PRIMARY KEY,
  owner bigint,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS asset_tags (
  id VARCHAR NOT NULL PRIMARY KEY,
  owner bigint,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS asset_tag_categories (
  tag VARCHAR NOT NULL,
  category VARCHAR NOT NULL,
  PRIMARY KEY (tag, category),
  FOREIGN KEY (tag) REFERENCES asset_tags (id) ON DELETE CASCADE,
  FOREIGN KEY (category) REFERENCES asset_categories (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS asset_tags_intermediate (
  tag VARCHAR NOT NULL,
  asset VARCHAR NOT NULL,
  PRIMARY KEY (tag, asset),
  FOREIGN KEY (tag) REFERENCES asset_tags (id) ON DELETE CASCADE,
  FOREIGN KEY (asset) REFERENCES assets (id) ON DELETE CASCADE
);
