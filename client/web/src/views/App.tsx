import React from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import Watchlist from "./Watchlist";
import Portfolio from "./Portfolio";
import Strategy from "./Strategy";
import Wallets from "./Wallets";

import "../../public/styles/app.css";

export default () => (
  <Tabs>
    <TabList>
      <Tab>Watchlist</Tab>
      <Tab>Portfolio</Tab>
      <Tab>Allocation</Tab>
      <Tab>Wallets</Tab>
    </TabList>

    <TabPanel>
      <Watchlist />
    </TabPanel>
    <TabPanel>
      <Portfolio />
    </TabPanel>
    <TabPanel>
      <Strategy />
    </TabPanel>
    <TabPanel>
      <Wallets />
    </TabPanel>
  </Tabs>
);
