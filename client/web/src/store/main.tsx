import { configureStore } from '@reduxjs/toolkit';

import uiReducer from './ui';
import portfolioReducer from './portfolio';
import watchlistsReducer from './watchlists';
import assetsReducer from './assets';

function saveToLocalStorage(state) {
  localStorage.setItem('theme', state.ui.theme);
}

const store = configureStore({
  reducer: {
    ui: uiReducer,
    watchlists: watchlistsReducer,
    portfolio: portfolioReducer,
    assets: assetsReducer,
  },
});

store.subscribe(() => saveToLocalStorage(store.getState()));

export default store;
