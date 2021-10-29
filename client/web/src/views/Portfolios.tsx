import React, { useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { makeStyles } from '@material-ui/core/styles';
import Tab from '@material-ui/core/Tab';
import TabContext from '@material-ui/lab/TabContext';
import TabList from '@material-ui/lab/TabList';
import TabPanel from '@material-ui/lab/TabPanel';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';
import Table from './utils/Table';
import {
  getUserPortfolios,
  createPortfolioThunk,
  deletePortfolioThunk,
} from '../store/portfolio';

import {
  preparePortfolio, preparePortfolios,
  interpolateColor,
} from './utils/Portfolios.tsx';

const useStyles = makeStyles({
  tabPanel: {
    padding: '2vh',
  },
  header: {
    padding: '15px 0',
  },
});

const tableStyle = {
  header: null,
};

export default () => {
  const classes = useStyles();
  const [tabIndex, setTabIndex] = React.useState('0');
  const portfolios = useSelector(getUserPortfolios);
  const dispatch = useDispatch();
  const data = [];
  data[0] = preparePortfolios(portfolios);

  const handleTabChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setTabIndex(newValue);
  };

  const handleCreatePortfolio = () => {
    dispatch(createPortfolioThunk(['slug1', 'Name1', 1]));
  };

  const handleDeletePortfolio = (id) => {
    dispatch(deletePortfolioThunk(id));
  };

  const tabs = [
    'Overall',
  ];

  portfolios.forEach((p) => {
    tabs.push(p.portfolio.slug);
    data.push(preparePortfolio(p));
  });

  return (
    <TabContext value={tabIndex}>
      <Paper>
        <TabList onChange={handleTabChange}>
          {tabs.map((tab, idx) => (
            <Tab label={tab} value={idx.toString()} key={idx.toString()} />
          ))}
        </TabList>
        <button onClick={handleCreatePortfolio}>Create</button>
      </Paper>
      {data.map((d, idx) => (
        <TabPanel key={idx.toString()} value={idx.toString()} className={classes.tabPanel}>
          <Typography variant="h6" className={classes.header}>
            {d.name}
          </Typography>
          <Table data={d} style={tableStyle} />
        </TabPanel>
      ))}
    </TabContext>
  );
};
