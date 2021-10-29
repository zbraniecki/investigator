import React from 'react';
import clsx from 'clsx';
import { makeStyles, createStyles, Theme } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';

import Drawer from '../Drawer';
import SettingsSet from '../SettingsSet';
import Watchlists from '../../Watchlists';
import Portfolios from '../../Portfolios';

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) => createStyles({
  root: {
    display: 'flex',
  },
  appBar: {
    display: 'flex',
    flexDirection: 'row',
    zIndex: theme.zIndex.drawer + 1,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
  },
  appBarShift: {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  },
  menuButton: {
    marginRight: 36,
  },
  appBarToolbar: {
    flex: 1,
  },
  hide: {
    display: 'none',
  },
  content: {
    flexGrow: 1,
    padding: theme.spacing(3),
    overflow: 'auto',
  },
}));

interface Props {
  menuItems: Array<any>,
  storedTheme: any,
  onThemeChange: any,
}

export default ({ menuItems, storedTheme, onThemeChange }: Props) => {
  const classes = useStyles();
  const [screenIndex, setScreenIndex] = React.useState(0);
  const [open, setOpen] = React.useState(false);

  const handleScreenChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setScreenIndex(newValue);
  };

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };

  let content;
  switch (screenIndex) {
    case 0:
      content = <Watchlists />;
      break;
    default:
      content = <Portfolios />;
      break;
  }

  return (
    <div className={classes.root}>
      <AppBar
        position="fixed"
        className={clsx(classes.appBar, {
          [classes.appBarShift]: open,
        })}
      >
        <Toolbar className={classes.appBarToolbar}>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            onClick={handleDrawerOpen}
            edge="start"
            className={clsx(classes.menuButton, {
              [classes.hide]: open,
            })}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap>
            Market Investigator
          </Typography>
        </Toolbar>
        <SettingsSet
          storedTheme={storedTheme}
          onThemeChange={onThemeChange}
        />
      </AppBar>
      <Toolbar />
      <Drawer
        selectedTab={screenIndex}
        onDrawerSelection={handleScreenChange}
        onDrawerClose={handleDrawerClose}
        menuItems={menuItems}
        open={open}
      />
      <main className={classes.content}>
        <Toolbar />
        { content }
      </main>
    </div>
  );
};
