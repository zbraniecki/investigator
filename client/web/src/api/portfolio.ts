const BASE_URL = 'http://127.0.0.1:8080/';

export const fetchPortfolios = async (user_id) => {
  const resp = user_id ?
    await fetch(`${BASE_URL}portfolio/filter?owner=${user_id}`) :
    await fetch(`${BASE_URL}portfolio/filter`);
  return resp.json();
};

export const createPortfolio = async ([slug, name, owner]) => {
  const resp = await fetch(`${BASE_URL}portfolio/create?slug=${slug}&name=${name}&owner=${owner}`);
  await resp.text();
  return fetchPortfolios(1);
};

export const deletePortfolio = async (id) => {
  const resp = await fetch(`${BASE_URL}portfolio/delete?id=${id}`);
  await resp.text();
  return fetchPortfolios(1);
};
