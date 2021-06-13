const BASE_URL = 'http://127.0.0.1:8080/';

export const fetchPortfolios = async () => {
  const resp = await fetch(`${BASE_URL}markets/portfolios`);
  return resp.json();
};
