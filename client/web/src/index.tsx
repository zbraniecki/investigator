import React from "react";
import { render } from "react-dom";
import App from "./views/App";
import { Provider } from 'react-redux';
import { store } from './store';

// render(<App/>, document.getElementById("root"));
render((
  <Provider store={store}>
    <App />
  </Provider>

), document.getElementById('root'));
