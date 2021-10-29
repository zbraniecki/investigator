export function interpolateColor(c0, c1, f){
  c0 = c0.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * (1 - f));
  c1 = c1.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * f);
  let ci = [0, 1, 2].map(i => Math.min(Math.round(c0[i] + c1[i]), 255));
  return ci.reduce((a, v) => ((a << 8) + v), 0).toString(16).padStart(6, '0');
}

let pf = new Intl.NumberFormat(undefined, {
  style: 'percent',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

let nf = new Intl.NumberFormat(undefined, {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

let cf = new Intl.NumberFormat(undefined, {
  style: 'currency',
  currency: 'USD',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

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

export function prepareWatchlist(allAssets, input) {
  let assets = [];
  for (let symbol of input.assets) {
    let asset = getAsset(allAssets, symbol);
    if (asset) {
      assets.push(asset);
    }
  }
  assets.sort((a, b) => {
    return a.market_cap_rank - b.market_cap_rank;
  });
  return assets.map((p) => {
    let change = p.price_change_percentage_24h / 100;
    let color = change > 0 ?
      interpolateColor('000000', '00FF00', change * 30)
      : interpolateColor('000000', 'FF0000', Math.abs(change) * 30);
    return {
      key: `${input.id}-${p.pair[0]}`,
      symbol: p.pair[0].toLocaleUpperCase(),
      value: cf.format(p.value),
      current_price: p.value,
      change: pf.format(change),
      price_change: change,
      color: `#${color}`,
    };
  });
}

