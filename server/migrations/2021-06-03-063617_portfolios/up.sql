CREATE TABLE IF NOT EXISTS portfolios (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  name VARCHAR,
  owner bigint,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS portfolio_assets (
  portfolio BIGSERIAL NOT NULL,
  asset VARCHAR NOT NULL,
  PRIMARY KEY (portfolio, asset),
  FOREIGN KEY (portfolio) REFERENCES portfolios (id) ON DELETE CASCADE,
  FOREIGN KEY (asset) REFERENCES assets (id) ON DELETE CASCADE
);
