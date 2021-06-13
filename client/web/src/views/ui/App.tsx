import React from 'react';
import { Provider } from 'react-redux';
import { useTheme } from '@material-ui/core/styles';
import useMediaQuery from '@material-ui/core/useMediaQuery';
import store from '../../store/main';

import UI from './UI';

export default () => {
  const theme = useTheme();
  const smallUI = useMediaQuery(theme.breakpoints.up('sm'), {
    noSsr: true,
  });

  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)', {
    noSsr: true,
  });
  return (
    <Provider store={store}>
      <UI
        prefersDarkMode={prefersDarkMode}
        smallUI={smallUI}
      />
    </Provider>
  );
};
