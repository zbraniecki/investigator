export const fetchWatchlists = async () => {
  let resp = await fetch("http://127.0.0.1:8081/account/watchlist");
  return await resp.json();
};

export const fetchPrices = async () => {
  let resp = await fetch("http://127.0.0.1:8080/oracle/prices");
  return await resp.json();
};

export const fetchRefreshPrices = async () => {
  let resp = await fetch("http://127.0.0.1:8080/oracle/prices?refresh=true");
  return await resp.json();
};

export const fetchPortfolios = async () => {
  let resp = await fetch("http://127.0.0.1:8081/account/portfolio");
  return await resp.json();
};

export const fetchStrategies = async () => {
  let resp = await fetch("http://127.0.0.1:8081/account/strategy");
  return await resp.json();
};

export const fetchWallets = async () => {
  let resp = await fetch("http://127.0.0.1:8080/oracle/wallets");
  return await resp.json();
};
