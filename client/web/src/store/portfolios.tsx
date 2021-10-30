import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { useDispatch } from 'react-redux';
import { fetchPortfolios, createPortfolio, deletePortfolio } from '../api/portfolios';

const USER_ID = 1;
export const fetchUserPortfoliosThunk = createAsyncThunk('portfolio/fetchUser', fetchPortfolios.bind(undefined, USER_ID));
export const createPortfolioThunk = createAsyncThunk('portfolio/create', createPortfolio);
export const deletePortfolioThunk = createAsyncThunk('portfolio/delete', deletePortfolio);

export const portfoliosSlice = createSlice({
  name: 'portfolios',
  initialState: {
    user: [],
  },
  reducers: {},
  extraReducers: {
    [fetchUserPortfoliosThunk.fulfilled]: (state, action) => {
      state.user = action.payload;
    },
    [createPortfolioThunk.fulfilled]: (state, action) => {
      state.user = action.payload;
    },
    [deletePortfolioThunk.fulfilled]: (state, action) => {
      state.user = action.payload;
    },
  },
});

export const getUserPortfolios = (state) => state.portfolios.user;

export default portfoliosSlice.reducer;
