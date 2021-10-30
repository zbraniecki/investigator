const BASE_URL = 'http://127.0.0.1:8000/';

export const fetchStrategies = async () => {
  const resp = await fetch(`${BASE_URL}strategy/list`);
  return resp.json();
};
