import React from 'react';
import { makeStyles, createStyles, Theme } from '@material-ui/core/styles';
import Tab from '@material-ui/core/Tab';
import TabContext from '@material-ui/lab/TabContext';
import TabList from '@material-ui/lab/TabList';
import TabPanel from '@material-ui/lab/TabPanel';
import Table from './Table';

const useStyles = makeStyles((theme: Theme) => createStyles({
}));

export default () => {
  const [value, setValue] = React.useState('0');
  const classes = useStyles();

  const handleChange = (event: React.ChangeEvent<{}>, newValue: string) => {
    setValue(newValue);
  };

  return (
    <TabContext value={value}>
      <TabList onChange={handleChange}>
        <Tab label="Crypto" value="0" />
        <Tab label="Stock" value="1" />
      </TabList>
      <TabPanel value="0">
        <Table kind="markets" />
      </TabPanel>
      <TabPanel value="1">
        <Table kind="stock" />
      </TabPanel>
    </TabContext>
  );
};
