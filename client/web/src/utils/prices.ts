export function getPrice(prices, symbol) {
  if (symbol == "usd") {
    return 1;
  }

  for (let price of prices) {
    if (price.pair[0] == symbol) {
      return price.value;
    }
  }
  return null;
}

export function getPrice2(prices, symbol) {
  if (symbol == "usd") {
    return {
      symbol: "usd",
      value: 1,
      market_cap: 0,
      price_change_percentage_24h: 0,
      market_cap_change_percentage_24h: 0,
    };
  }

  for (let price of prices) {
    if (price.pair[0] == symbol) {
      return price;
    }
  }
  return null;
}

