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

