const BASE_URL = 'http://127.0.0.1:8000/';

export const fetchWatchlists = async (user_id) => {
  const resp = await fetch(`${BASE_URL}oracle/watchlists`);
  return resp.json();
};
