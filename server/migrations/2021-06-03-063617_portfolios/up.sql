CREATE TABLE IF NOT EXISTS portfolios (
  id SERIAL NOT NULL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  name VARCHAR,
  owner INTEGER,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS portfolio_assets (
  portfolio SERIAL NOT NULL,
  asset VARCHAR NOT NULL,
  PRIMARY KEY (portfolio, asset),
  FOREIGN KEY (portfolio) REFERENCES portfolios (id) ON DELETE CASCADE,
  FOREIGN KEY (asset) REFERENCES assets (id) ON DELETE CASCADE
);
