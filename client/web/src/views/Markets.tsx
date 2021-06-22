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
  getPublicPortfolios,
} from '../store/portfolio';

function interpolateColor(c0, c1, f){
    c0 = c0.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * (1-f))
    c1 = c1.match(/.{1,2}/g).map((oct)=>parseInt(oct, 16) * f)
    let ci = [0,1,2].map(i => Math.min(Math.round(c0[i]+c1[i]), 255))
    return ci.reduce((a,v) => ((a << 8) + v), 0).toString(16).padStart(6, "0")
}

const useStyles = makeStyles({
  tabPanel: {
    padding: '2vh',
  },
  header: {
    padding: '15px 0',
  },
});

let pf = new Intl.NumberFormat(undefined, {
  style: "percent",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

let cf = new Intl.NumberFormat(undefined, {
  style: "currency",
  currency: "USD",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

function preparePortfolios(input) {
  return input.map((p) => {
    const sub = preparePortfolio(p, 10);
    return {
      key: p.portfolio.id,
      symbol: p.portfolio.slug,
      value: 0,
      change: 0.0,
      sub,
    };
  });
}

function preparePortfolio(input, max) {
  let sorted = Array.from(input.assets);
  sorted.sort((a, b) => {
    return a.info.market_cap_rank - b.info.market_cap_rank;
  });
  if (max) {
    sorted = sorted.slice(0, max);
  }
  return sorted.map((p) => {
    let change = p.info.price_change_percentage_24h_in_currency / 100;
    let color = change > 0 ?
      interpolateColor("000000", "00FF00", change * 30)
      : interpolateColor("000000", "FF0000", Math.abs(change) * 30);
    return {
      key: p.asset.id,
      symbol: p.asset.symbol.toLocaleUpperCase(),
      value: cf.format(p.info.current_price),
      change: pf.format(change),
      color: `#${color}`,
    };
  });
}

const tableStyle = {
  header: null,
};

export default () => {
  const classes = useStyles();
  const [tabIndex, setTabIndex] = React.useState('0');
  const portfolios = useSelector(getPublicPortfolios);
  const data = [];
  data[0] = preparePortfolios(portfolios);

  const handleTabChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setTabIndex(newValue);
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
