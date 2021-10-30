import React from 'react';
import { makeStyles, useTheme } from '@material-ui/core/styles';
import BottomNavigation from '@material-ui/core/BottomNavigation';
import BottomNavigationAction from '@material-ui/core/BottomNavigationAction';
import Container from '@material-ui/core/Container';
import Box from '@material-ui/core/Box';
import AppBar from '@material-ui/core/AppBar';

import SettingsSet from '../SettingsSet';
import Watchlists from '../../Watchlists';
import Portfolios from '../../Portfolios';
import Strategies from '../../Strategies';

const useStyles = makeStyles({
  root: {
    display: 'flex',
    flexDirection: 'column',
    height: '100vh',
    padding: '0',
  },
  appBar: {
    display: 'flex',
    flexDirection: 'row-reverse',
    alignItems: 'end',
  },
  box: {
    flex: 1,
    overflow: 'auto',
    scrollbarWidth: 'none',
  },
  bottomNav: {
  },
});

interface Props {
  menuItems: Array<any>,
  storedTheme: any,
  onThemeChange: any,
}

export default ({ menuItems, storedTheme, onThemeChange }: Props) => {
  const classes = useStyles();
  const theme = useTheme();
  const [screenIndex, setScreenIndex] = React.useState(0);

  const handleScreenChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setScreenIndex(newValue);
  };

  let content;
  switch (screenIndex) {
    case 0:
      content = <Watchlists />;
      break;
    case 1:
      content = <Portfolios />;
      break;
    case 2:
      content = <Strategies />;
      break;
    default:
      content = <Watchlists />;
      break;
  }

  return (
    <Container maxWidth="sm" className={classes.root}>
      <AppBar position="static" className={classes.appBar}>
        <SettingsSet
          storedTheme={storedTheme}
          onThemeChange={onThemeChange}
        />
      </AppBar>
      <Box className={classes.box}>
        { content }
      </Box>
      <Box borderTop={1} borderColor={theme.palette.divider}>
        <BottomNavigation
          value={screenIndex}
          onChange={handleScreenChange}
          className={classes.bottomNav}
          showLabels
        >
          {menuItems.map((menuItem) => (
            <BottomNavigationAction key={menuItem[0]} label={menuItem[0]} icon={menuItem[1]} />
          ))}
        </BottomNavigation>
      </Box>
    </Container>
  );
};
