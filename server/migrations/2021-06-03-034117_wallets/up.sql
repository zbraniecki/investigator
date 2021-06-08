CREATE TABLE IF NOT EXISTS services (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  url VARCHAR,
  owner INTEGER,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS wallets (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR  NOT NULL,
  url VARCHAR,
  service VARCHAR,
  owner INTEGER,
  FOREIGN KEY (owner) REFERENCES identities (id) ON DELETE CASCADE,
  FOREIGN KEY (service) REFERENCES services (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS wallet_yield_kinds (
  id VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS wallet_yields (
  service VARCHAR,
  wallet VARCHAR,
  asset VARCHAR NOT NULL,
  kind VARCHAR NOT NULL,
  apy_lower_bound DOUBLE PRECISION NOT NULL,
  apy_upper_bound DOUBLE PRECISION,
  start_date TIMESTAMP,
  end_date TIMESTAMP,
  PRIMARY KEY (wallet, service, asset),
  FOREIGN KEY (wallet) REFERENCES wallets (id) ON DELETE CASCADE,
  FOREIGN KEY (service) REFERENCES services (id) ON DELETE CASCADE,
  FOREIGN KEY (asset) REFERENCES assets (id) ON DELETE CASCADE,
  FOREIGN KEY (kind) REFERENCES wallet_yield_kinds (id) ON DELETE SET NULL
);
