
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

function getAsset(allAssets, symbol) {
  for (let asset of allAssets) {
    if (asset.pair[0] == symbol) {
      return asset;
    }
  }
  return null;
}


export function prepareStrategy(allAssets, input) {
  let targets = [];
  for (let target of input.targets) {
    let asset = getAsset(allAssets, target.symbol);
    if (asset) {
      targets.push({
        target,
        asset,
      });
    }
  }
  targets.sort((a, b) => {
    return b.target.percent - a.target.percent;
  });

  return targets.map((p) => {
    let change = p.asset.price_change_percentage_24h / 100;
    let color = change > 0 ?
      interpolateColor('000000', '00FF00', change * 30)
      : interpolateColor('000000', 'FF0000', Math.abs(change) * 30);
    return {
      key: `${input.id}-${p.target.symbol}`,
      symbol: p.asset.pair[0].toLocaleUpperCase(),
      value: cf.format(p.asset.value),
      target: pf.format(p.target.percent),
      percent: pf.format(0.1),
      current_price: p.asset.value,
      change: pf.format(change),
      price_change: change,
      color: `#${color}`,
    };
  });
}
