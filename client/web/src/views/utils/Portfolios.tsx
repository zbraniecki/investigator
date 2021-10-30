import fmt from './Formatters';

export function interpolateColor(c0, c1, f){
  c0 = c0.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * (1 - f));
  c1 = c1.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * f);
  let ci = [0, 1, 2].map(i => Math.min(Math.round(c0[i] + c1[i]), 255));
  return ci.reduce((a, v) => ((a << 8) + v), 0).toString(16).padStart(6, '0');
}

export function preparePortfolios(input) {
  return input.map((p) => {
    const sub = preparePortfolio(p, 10);

    const sum = 0;
    for (let s of sub) {
      sum += s.current_price;
    }

    const old_sum = 0;
    for (let s of sub) {
      old_sum += s.current_price * s.price_change;
    }
    let change = old_sum / sum;
    let color = change > 0 ?
      interpolateColor('000000', '00FF00', change * 30)
      : interpolateColor('000000', 'FF0000', Math.abs(change) * 30);
    return {
      key: p.portfolio.id,
      symbol: p.portfolio.slug,
      value: nf.format(sum),
      change: pf.format(change),
      color: `#${color}`,
      sub,
    };
  });
}

function getAsset(allAssets, symbol) {
  for (let asset of allAssets) {
    if (asset.pair[0] == symbol) {
      return asset;
    }
  }
  return null;
}


export function preparePortfolio(allAssets, input, displayValues) {
  let holdings = [];
  for (let holding of input.holdings) {
    let asset = getAsset(allAssets, holding.symbol);
    if (asset) {
      holdings.push({
        holding,
        asset,
      });
    }
  }
  holdings.sort((a, b) => {
    let valueA = a.asset.value * a.holding.quantity;
    let valueB = b.asset.value * b.holding.quantity;
    return valueB - valueA;
  });

  return holdings.map((p) => {
    let change = p.asset.price_change_percentage_24h / 100;
    let color = change > 0 ?
      interpolateColor('000000', '00FF00', change * 30)
      : interpolateColor('000000', 'FF0000', Math.abs(change) * 30);
    return {
      rank: 42,
      key: `${input.id}-${p.holding.wallet}-${p.holding.quantity}-${p.asset.pair[0]}`,
      symbol: p.asset.pair[0].toLocaleUpperCase(),
      quantity: displayValues ? fmt.number(p.holding.quantity) : `--.--`,
      value: displayValues ? fmt.currency(p.holding.quantity * p.asset.value) : `$--.--`,
      current_price: p.asset.value,
      change: fmt.percent(change),
      price_change: change,
      color: `#${color}`,
    };
  });
}
