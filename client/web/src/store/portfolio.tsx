import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchPortfolios } from '../api/portfolio';

export const fetchPublicPortfoliosThunk = createAsyncThunk('portfolio/fetchPublic', fetchPortfolios);

export const portfolioSlice = createSlice({
  name: 'portfolio',
  initialState: {
    public: [],
  },
  reducers: {},
  extraReducers: {
    [fetchPublicPortfoliosThunk.fulfilled]: (state, action) => {
      state.public = action.payload;
    },
  },
});

export const getPublicPortfolios = (state) => state.portfolio.public;

export default portfolioSlice.reducer;
