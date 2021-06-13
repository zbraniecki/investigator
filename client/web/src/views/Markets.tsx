import React from 'react';
import { useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core/styles';
import Tab from '@material-ui/core/Tab';
import TabContext from '@material-ui/lab/TabContext';
import TabList from '@material-ui/lab/TabList';
import TabPanel from '@material-ui/lab/TabPanel';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import Table from './utils/Table';

import {
  getPortfolios,
} from '../store/markets';

const useStyles = makeStyles({
  tabPanel: {
    padding: '2vh',
  },
  header: {
    padding: '15px 0',
  },
});

function preparePortfolios(input) {
  return input.map((p) => {
    const sub = p.assets.map((a) => ({
      key: a,
      symbol: a,
      value: 0,
      change: 0.0,
    }));
    return {
      key: p.name,
      symbol: p.name,
      value: 0,
      change: 0.0,
      sub,
    };
  });
}

function preparePortfolio(input) {
  return input.map((p) => ({
    key: p,
    symbol: p,
    value: 0,
    change: 0.0,
  }));
}

const data = {
  overall: [],
  's&p500': [
    {
      key: 'IBM',
      symbol: 'IBM',
      value: 12121,
      change: 0.2,
    },
    {
      key: 'TSLA',
      symbol: 'TSLA',
      value: 9212,
      change: 0.1,
    },
  ],
  crypto: [],
};

const tableStyle = {
  header: null,
};

export default () => {
  const classes = useStyles();
  const [tabIndex, setTabIndex] = React.useState('0');
  const portfolios = useSelector(getPortfolios);
  data.overall = preparePortfolios(portfolios);
  for (const p of portfolios) {
    if (p.name === 'crypto') {
      data.crypto = preparePortfolio(p.assets);
    }
  }

  const handleTabChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setTabIndex(newValue);
  };

  return (
    <TabContext value={tabIndex}>
      <Paper>
        <TabList onChange={handleTabChange}>
          <Tab label="Overall" value="0" />
          <Tab label="S&P 500" value="1" />
          <Tab label="Crypto" value="2" />
        </TabList>
      </Paper>
      <TabPanel value="0" className={classes.tabPanel}>
        <Typography variant="h6" className={classes.header}>
        &nbsp;
        </Typography>
        <Table data={data.overall} style={tableStyle} />
      </TabPanel>
      <TabPanel value="1" className={classes.tabPanel}>
        <Typography variant="h6" className={classes.header}>
          S&P 500
        </Typography>
        <Table data={data['s&p500']} style={tableStyle} />
      </TabPanel>
      <TabPanel value="2" className={classes.tabPanel}>
        <Typography variant="h6" className={classes.header}>
          Crypto
        </Typography>
        <Table data={data.crypto} style={tableStyle} />
      </TabPanel>
    </TabContext>
  );
};
