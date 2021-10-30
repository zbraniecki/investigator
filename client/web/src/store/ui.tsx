import { createSlice } from '@reduxjs/toolkit';

export enum Theme {
  Automatic,
  Light,
  Dark,
}

let theme = localStorage.getItem('theme');
if (theme === undefined) {
  theme = Theme.Automatic;
} else {
  theme = parseInt(theme, 10);
}

export const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    theme: theme || Theme.Automatic,
    displayValues: true,
  },
  reducers: {
    setTheme: (state, { payload }) => {
      state.theme = payload;
    },
    setDisplayValues: (state, { payload }) => {
      state.displayValues = payload;
    },
  },
  extraReducers: {},
});

export const getDisplayValues = (state) => state.ui.displayValues;
export const getTheme = (state) => state.ui.theme;

export const { setTheme, setDisplayValues } = uiSlice.actions;
export default uiSlice.reducer;
