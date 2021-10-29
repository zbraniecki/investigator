import React, { useEffect } from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import { ThemeProvider, createTheme } from '@material-ui/core/styles';
import cyan from '@material-ui/core/colors/cyan';
import { useSelector, useDispatch } from 'react-redux';

import AccountBalanceIcon from '@material-ui/icons/AccountBalance';
import TrendingUpIcon from '@material-ui/icons/TrendingUp';
import PieChartIcon from '@material-ui/icons/PieChart';
import {
  getTheme,
  setTheme,
  Theme,
} from '../../store/ui';
import {
  fetchPublicWatchlistsThunk,
} from '../../store/watchlists';
import {
  fetchAssetsThunk,
} from '../../store/assets';

import SmallChrome from './chrome/Small';
import LargeChrome from './chrome/Large';

const menuItems = [
  ['Watchlists', <AccountBalanceIcon />],
  ['Portfolios', <TrendingUpIcon />],
  ['Strategies', <PieChartIcon />],
];

interface Props {
  prefersDarkMode: bool,
  smallUI: bool,
}

export default ({ prefersDarkMode, smallUI }: Props) => {
  const storedTheme = useSelector(getTheme);
  const dispatch = useDispatch();

  let themeName;
  switch (storedTheme) {
    case Theme.Light:
      themeName = 'light';
      break;
    case Theme.Dark:
      themeName = 'dark';
      break;
    case Theme.Automatic:
    default:
      themeName = prefersDarkMode ? 'dark' : 'light';
      break;
  }

  const theme = React.useMemo(
    () => createTheme({
      palette: {
        primary: cyan,
        type: themeName,
      },
    }),
    [themeName],
  );

  function onThemeChange() {
    let newValue;
    switch (storedTheme) {
      case Theme.Automatic:
        newValue = Theme.Light;
        break;
      case Theme.Light:
        newValue = Theme.Dark;
        break;
      case Theme.Dark:
      default:
        newValue = Theme.Automatic;
        break;
    }
    dispatch(setTheme(newValue));
  }

  useEffect(() => {
    dispatch(fetchPublicWatchlistsThunk());
    dispatch(fetchAssetsThunk());
  }, [dispatch]);

  const chrome = smallUI
    ? (
      <LargeChrome
        menuItems={menuItems}
        storedTheme={storedTheme}
        onThemeChange={onThemeChange}
      />
    )
    : (
      <SmallChrome
        menuItems={menuItems}
        storedTheme={storedTheme}
        onThemeChange={onThemeChange}
      />
    );

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      { chrome }
    </ThemeProvider>
  );
};
