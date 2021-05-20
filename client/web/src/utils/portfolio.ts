import {
  getPrice
} from './prices';

export function computePortfolio(portfolio, prices) {
  let aggr = {};

  for (let holding of portfolio.holdings) {
    let symbol = holding.symbol;
    let wallet = holding.locked ?
     `${holding.wallet} (${holding.locked})` :
     holding.wallet;
    if (!aggr.hasOwnProperty(symbol)) {
      aggr[symbol] = [{
        quantity: holding.quantity,
        wallet: wallet,
        wallet_name: holding.wallet,
      }];
    } else {
      aggr[symbol].push({
        quantity: holding.quantity,
        wallet: wallet,
        wallet_name: holding.wallet,
      });
    }
  }

  let results = [];

  for (let [key, value] of Object.entries(aggr)) {
    let sum = value.reduce((a, b) => a + b.quantity, 0);
    let price = getPrice(prices, key);

    let subRows = value.map((v) => {
      return {
        symbol: "",
        quantity: v.quantity,
        wallet: v.wallet,
        wallet_name: v.wallet_name,
        value: price * v.quantity,
      };
    }).sort((a, b) => b.quantity - a.quantity);

    let wallet = new Set();
    for (let row of subRows) {
      if (row.wallet === null) {
        continue;
      }
      if (row.value > price * sum * 0.1) {
        wallet.add(row.wallet);
      }
    }

    if (subRows.length < 2) {
      subRows = undefined;
    }

    results.push({
      symbol: key,
      quantity: sum,
      value: price * sum,
      wallet: Array.from(wallet).join(", "),
      subRows,
    });
  }

  results.sort((a, b) => b.value - a.value);
  return results;
}
