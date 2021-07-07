const BASE_URL = 'http://127.0.0.1:8080/';

export const fetchPortfolios = async (user_id) => {
  const resp = user_id ?
    await fetch(`${BASE_URL}portfolio/filter?owner=${user_id}`) :
    await fetch(`${BASE_URL}portfolio/filter`);
  return resp.json();
};

export const createPortfolio = async (slug, name, owner) => {
  console.log(slug);
  console.log(name);
  console.log(owner);
  // const resp = user_id ?
  //   await fetch(`${BASE_URL}portfolio/filter?owner=${user_id}`) :
  //   await fetch(`${BASE_URL}portfolio/filter`);
  // return resp.json();
};
