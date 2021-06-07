import React from 'react';
import { makeStyles, createStyles, Theme } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';

import Markets from './Markets';
import Drawer from './Drawer';

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) => createStyles({
  root: {
    display: 'flex',
  },
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
  },
  content: {
    flexGrow: 1,
    padding: theme.spacing(3),
    marginTop: '64px',
  },
}));

export default () => {
  const [selectedTab, setSelectedTab] = React.useState(0);
  const classes = useStyles();

  const handleListItemClick = (
    event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    index: number,
  ) => {
    setSelectedTab(index);
  };

  return (
    <div className={classes.root}>
      <AppBar className={classes.appBar}>
        <Toolbar>
          <Typography variant="h6" noWrap>
            Market Investigator
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Drawer selectedTab={selectedTab} onDrawerSelection={handleListItemClick} />
      <main className={classes.content}>
        { selectedTab === 0 ? <Markets /> : <div>Foo</div> }
      </main>
    </div>
  );
};
