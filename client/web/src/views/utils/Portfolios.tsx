
export function interpolateColor(c0, c1, f){
    c0 = c0.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * (1-f))
    c1 = c1.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * f)
    let ci = [0,1,2].map(i => Math.min(Math.round(c0[i]+c1[i]), 255))
    return ci.reduce((a,v) => ((a << 8) + v), 0).toString(16).padStart(6, "0")
}

let pf = new Intl.NumberFormat(undefined, {
  style: "percent",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

let nf = new Intl.NumberFormat(undefined, {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

let cf = new Intl.NumberFormat(undefined, {
  style: "currency",
  currency: "USD",
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
      interpolateColor("000000", "00FF00", change * 30)
      : interpolateColor("000000", "FF0000", Math.abs(change) * 30);
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

export function preparePortfolio(input, max) {
  let sorted = Array.from(input.assets);
  sorted.sort((a, b) => {
    return a.info.market_cap_rank - b.info.market_cap_rank;
  });
  if (max) {
    sorted = sorted.slice(0, max);
  }
  return sorted.map((p) => {
    let change = p.info.price_change_percentage_24h_in_currency / 100;
    let color = change > 0 ?
      interpolateColor("000000", "00FF00", change * 30)
      : interpolateColor("000000", "FF0000", Math.abs(change) * 30);
    return {
      key: p.asset.id,
      symbol: p.asset.symbol.toLocaleUpperCase(),
      value: cf.format(p.info.current_price),
      current_price: p.info.current_price,
      change: pf.format(change),
      price_change: change,
      color: `#${color}`,
    };
  });
}

