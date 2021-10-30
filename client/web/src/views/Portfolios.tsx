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
import fmt from './utils/Formatters';
import {
  getAssets,
} from '../store/assets';
import {
  getUserPortfolios,
  createPortfolioThunk,
  deletePortfolioThunk,
} from '../store/portfolios';
import {
  getPortfolioValue,
} from '../store/system';

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
  nested: false,
  header: true,
  columns: [
    { label: 'Rank', align: 'left', value: {
      key: 'rank',
    } },
    { label: 'Symbol', align: 'left', value: {
      key: 'symbol',
    } },
    { label: 'Quantity', align: 'right', value: {
      key: 'quantity',
    } },
    { label: 'Value', align: 'right', value: {
      key: 'value',
    } },
  ],
};

export default () => {
  const classes = useStyles();
  const [tabIndex, setTabIndex] = React.useState('0');
  const portfolios = useSelector(getUserPortfolios);
  const assets = useSelector(getAssets);

  const dispatch = useDispatch();

  const data = [];
  const tabs = [];
  const values = [];

  for (let portfolio of portfolios) {
    tabs.push(portfolio.name);
    data.push(preparePortfolio(assets, portfolio));

    //XXX: Should we fetch all portfolio values at once?
    let value = useSelector(getPortfolioValue); // XXX: How to pass portfolio id?
    values.push(value);
  }

  const handleTabChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setTabIndex(newValue);
  };

  // const handleCreatePortfolio = () => {
  //   dispatch(createPortfolioThunk(['slug1', 'Name1', 1]));
  // };

  // const handleDeletePortfolio = (id) => {
  //   dispatch(deletePortfolioThunk(id));
  // };

  // portfolios.forEach((p) => {
  //   tabs.push(p.portfolio.slug);
  //   data.push(preparePortfolio(p));
  // });


  return (
    <TabContext value={tabIndex}>
      <Paper>
        <TabList onChange={handleTabChange}>
          {tabs.map((tab, idx) => (
            <Tab label={tab} value={idx.toString()} key={idx.toString()} />
          ))}
        </TabList>
      </Paper>
      {data.map((d, idx) => (
        <TabPanel key={idx.toString()} value={idx.toString()} className={classes.tabPanel}>
          <Typography variant="h6" className={classes.header} align="right">
            {fmt.currency(values[idx])}
          </Typography>
          <Table data={d} style={tableStyle} />
        </TabPanel>
      ))}
    </TabContext>
  );
};
