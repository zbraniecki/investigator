import { configureStore } from '@reduxjs/toolkit';

import uiReducer from './ui';

function saveToLocalStorage(state) {
  localStorage.setItem('theme', state.ui.theme);
}

const store = configureStore({
  reducer: {
    ui: uiReducer,
  },
});

store.subscribe(() => saveToLocalStorage(store.getState()));

export default store;
