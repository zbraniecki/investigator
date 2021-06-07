import React from 'react';
import { makeStyles, createStyles, Theme } from '@material-ui/core/styles';
import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import Divider from '@material-ui/core/Divider';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import AccountBalanceIcon from '@material-ui/icons/AccountBalance';
import TrendingUpIcon from '@material-ui/icons/TrendingUp';
import SettingsIcon from '@material-ui/icons/Settings';
import Markets from './Markets';

const drawerWidth = 240;
const menuItems = [
  ['Markets', <AccountBalanceIcon />],
  ['Portfolios', <TrendingUpIcon />],
];

const menuItemsLower = [
  ['Settings', <SettingsIcon />],
];

const useStyles = makeStyles((theme: Theme) => createStyles({
  root: {
    display: 'flex',
  },
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
  },
  drawer: {
    width: drawerWidth,
    flexShrink: 0,
  },
  drawerPaper: {
    width: drawerWidth,
    marginTop: '64px',
  },
  drawerContainer: {
    overflow: 'auto',
  },
  content: {
    flexGrow: 1,
    padding: theme.spacing(3),
    marginTop: '64px',
  },
}));

export default (props) => {
  const classes = useStyles();
  const { onDrawerSelection } = props;
  const { selectedTab } = props;

  return (
    <Drawer
      className={classes.drawer}
      variant="permanent"
      classes={{
        paper: classes.drawerPaper,
      }}
    >
      <div className={classes.drawerContainer}>
        <List>
          {menuItems.map((menuItem, index) => (
            <ListItem
              button
              key={menuItem[0]}
              selected={selectedTab === index}
              onClick={(event) => onDrawerSelection(event, index)}
            >
              <ListItemIcon>{menuItem[1]}</ListItemIcon>
              <ListItemText primary={menuItem[0]} />
            </ListItem>
          ))}
        </List>
        <Divider />
        <List>
          {menuItemsLower.map((menuItem, index) => (
            <ListItem button key={menuItem[0]}>
              <ListItemIcon>{menuItem[1]}</ListItemIcon>
              <ListItemText primary={menuItem[0]} />
            </ListItem>
          ))}
        </List>
      </div>
    </Drawer>
  );
};
