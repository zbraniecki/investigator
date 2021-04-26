export const fetchWatchlists = async () => {
  let resp = await fetch("http://127.0.0.1:8081/account/watchlist");
  return await resp.json();
};

export const fetchPrices = async () => {
  let resp = await fetch("http://127.0.0.1:8080/oracle/prices");
  return await resp.json();
};
