import { configureStore } from '@reduxjs/toolkit';

import uiReducer from './ui';
import marketsReducer from './markets';

function saveToLocalStorage(state) {
  localStorage.setItem('theme', state.ui.theme);
}

const store = configureStore({
  reducer: {
    ui: uiReducer,
    markets: marketsReducer,
  },
});

store.subscribe(() => saveToLocalStorage(store.getState()));

export default store;
