import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { useDispatch } from 'react-redux';
import { fetchPortfolios, createPortfolio, deletePortfolio } from '../api/portfolio';

export const fetchPublicPortfoliosThunk = createAsyncThunk('portfolio/fetchPublic', fetchPortfolios);
const USER_ID = 1;
export const fetchUserPortfoliosThunk = createAsyncThunk('portfolio/fetchUser', fetchPortfolios.bind(undefined, USER_ID));
export const createPortfolioThunk = createAsyncThunk('portfolio/create', createPortfolio);
export const deletePortfolioThunk = createAsyncThunk('portfolio/delete', deletePortfolio);

export const portfolioSlice = createSlice({
  name: 'portfolio',
  initialState: {
    public: [],
    user: [],
  },
  reducers: {},
  extraReducers: {
    [fetchPublicPortfoliosThunk.fulfilled]: (state, action) => {
      state.public = action.payload;
    },
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

export const getPublicPortfolios = (state) => state.portfolio.public;
export const getUserPortfolios = (state) => state.portfolio.user;

export default portfolioSlice.reducer;
