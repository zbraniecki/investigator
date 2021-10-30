import { configureStore } from '@reduxjs/toolkit';

import uiReducer from './ui';
import portfoliosReducer from './portfolios';
import watchlistsReducer from './watchlists';
import strategiesReducer from './strategies';
import assetsReducer from './assets';
import systemReducer from './system';

function saveToLocalStorage(state) {
  localStorage.setItem('theme', state.ui.theme);
}

const store = configureStore({
  reducer: {
    ui: uiReducer,
    watchlists: watchlistsReducer,
    portfolios: portfoliosReducer,
    strategies: strategiesReducer,
    assets: assetsReducer,
    system: systemReducer,
  },
});

store.subscribe(() => saveToLocalStorage(store.getState()));

export default store;
