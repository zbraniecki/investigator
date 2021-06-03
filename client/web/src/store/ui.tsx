import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';

export enum Theme {
  Automatic,
  Light,
  Dark,
}

export const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    theme: Theme.Automatic,
    displayValues: true,
  },
  reducers: {},
  extraReducers: {}
});

export const getDisplayValues = state => state.ui.displayValues;
export const getTheme = state => state.ui.theme;

export default uiSlice.reducer;
