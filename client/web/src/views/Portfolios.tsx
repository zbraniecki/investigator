import React from 'react';
import Tab from '@material-ui/core/Tab';
import TabContext from '@material-ui/lab/TabContext';
import TabList from '@material-ui/lab/TabList';
import TabPanel from '@material-ui/lab/TabPanel';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

export default () => {
  const [tabIndex, setTabIndex] = React.useState('0');

  const handleTabChange = (event: React.ChangeEvent<{}>, newValue: number) => {
    setTabIndex(newValue);
  };

  return (
    <TabContext value={tabIndex}>
      <Paper>
        <TabList onChange={handleTabChange}>
          <Tab label="Overall" value="0" />
          <Tab label="Crypto" value="1" />
        </TabList>
      </Paper>
      <TabPanel value="0">
        <Typography variant="h6">
          Overall
        </Typography>
      </TabPanel>
      <TabPanel value="1">
        <Typography variant="h6">
          Crypto
        </Typography>
      </TabPanel>
    </TabContext>
  );
};
