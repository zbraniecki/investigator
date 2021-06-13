import React from 'react';
import { Provider } from 'react-redux';
import useMediaQuery from '@material-ui/core/useMediaQuery';
import store from '../../store/main';

import UI from './UI';

export default () => {
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)', {
    noSsr: true,
  });
  return (
    <Provider store={store}>
      <UI
        prefersDarkMode={prefersDarkMode}
      />
    </Provider>
  );
};
