import Settings from "./Settings";
import Tabs from "./Tabs";
import React from "react";

export default () => (
  <React.Fragment>
    <header>
      <h1>Markets Investigator</h1>
      <Settings />
    </header>
    <section>
      <Tabs />
    </section>
    <footer>
      Last Sync: 2001-01-01 14:34
    </footer>
  </React.Fragment>
);
