const BASE_URL = 'http://127.0.0.1:8000/';

export const fetchAssets = async () => {
  const resp = await fetch(`${BASE_URL}oracle/assets`);
  return resp.json();
};
