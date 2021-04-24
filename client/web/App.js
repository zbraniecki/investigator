import "./styles.css";

import React from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

import Market from "./Market";
import Portfolio from "./Portfolio";
import Allocation from "./Allocation";

export default () => (
  <Tabs>
    <TabList>
      <Tab>Market</Tab>
      <Tab>Portfolio</Tab>
      <Tab>Allocation</Tab>
    </TabList>

    <TabPanel>
      <Market />
    </TabPanel>
    <TabPanel>
      <Portfolio />
    </TabPanel>
    <TabPanel>
      <Allocation />
    </TabPanel>
  </Tabs>
);
