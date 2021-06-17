    /* https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin%2Cethereum&order=market_cap_desc&per_page=5&page=1&sparkline=false&price_change_percentage=1h%2C24h%2C7d%2C14d%2C30d%2C200d%2C1y */
    /* { */
    /* "id": "bitcoin", */
    /* "symbol": "btc", */
    /* "name": "Bitcoin", */
    /* "image": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579", */
    /* "current_price": 40559, */
    /* "market_cap": 760608851294, */
    /* "market_cap_rank": 1, */
    /* "fully_diluted_valuation": 852550904461, */
    /* "total_volume": 43869649789, */
    /* "high_24h": 41117, */
    /* "low_24h": 39042, */
    /* "price_change_24h": 1160.44, */
    /* "price_change_percentage_24h": 2.9454, */
    /* "market_cap_change_24h": 22863028086, */
    /* "market_cap_change_percentage_24h": 3.09904, */
    /* "circulating_supply": 18735287, */
    /* "total_supply": 21000000, */
    /* "max_supply": 21000000, */
    /* "ath": 64805, */
    /* "ath_change_percentage": -37.41371, */
    /* "ath_date": "2021-04-14T11:54:46.763Z", */
    /* "atl": 67.81, */
    /* "atl_change_percentage": 59713.40413, */
    /* "atl_date": "2013-07-06T00:00:00.000Z", */
    /* "roi": null, */
    /* "last_updated": "2021-06-15T06:40:05.123Z", */
    /* "price_change_percentage_14d_in_currency": 8.618461155863656, */
    /* "price_change_percentage_1h_in_currency": 0.42538793328146973, */
    /* "price_change_percentage_1y_in_currency": 333.9721897435015, */
    /* "price_change_percentage_200d_in_currency": 136.66006161606012, */
    /* "price_change_percentage_24h_in_currency": 2.945403029783454, */
    /* "price_change_percentage_30d_in_currency": -13.300365735467231, */
    /* "price_change_percentage_7d_in_currency": 21.247216326334467 */
  /* }, */
CREATE TABLE IF NOT EXISTS assets_info (
  asset VARCHAR NOT NULL,
  reference_asset VARCHAR NOT NULL,
  current_price DOUBLE PRECISION,
  market_cap bigint,
  market_cap_rank bigint,
  total_volume bigint,
  high_24h DOUBLE PRECISION,
  low_24h DOUBLE PRECISION,
  price_change_24h DOUBLE PRECISION,
  market_cap_change_24h DOUBLE PRECISION,
  market_cap_change_percentage_24h DOUBLE PRECISION,
  circulating_supply DOUBLE PRECISION,
  total_supply DOUBLE PRECISION,
  max_supply DOUBLE PRECISION,
  ath DOUBLE PRECISION,
  ath_change_percentage DOUBLE PRECISION,
  ath_date TIMESTAMP with time zone,
  atl DOUBLE PRECISION,
  atl_change_percentage DOUBLE PRECISION,
  atl_date TIMESTAMP with time zone,
  last_updated TIMESTAMP with time zone,
  price_change_percentage_1h DOUBLE PRECISION,
  price_change_percentage_24h DOUBLE PRECISION,
  price_change_percentage_7d DOUBLE PRECISION,
  price_change_percentage_14d DOUBLE PRECISION,
  price_change_percentage_30d DOUBLE PRECISION,
  price_change_percentage_200d DOUBLE PRECISION,
  price_change_percentage_1y DOUBLE PRECISION,

  PRIMARY KEY (asset, reference_asset),
  FOREIGN KEY (asset) REFERENCES assets (id) ON DELETE CASCADE,
  FOREIGN KEY (reference_asset) REFERENCES assets (id) ON DELETE CASCADE
);
