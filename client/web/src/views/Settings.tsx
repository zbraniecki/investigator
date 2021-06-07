import { useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import {
  Theme,
  getTheme,
} from '../store/ui';

export default () => {
  const theme = useSelector(getTheme);
  const dispatch = useDispatch();

  function toggleTheme() {
    let newTheme;
    switch (theme) {
      case Theme.Automatic:
        newTheme = Theme.Light;
        break;
      case Theme.Light:
        newTheme = Theme.Dark;
        break;
      case Theme.Dark:
        newTheme = Theme.Automatic;
        break;
    }
    dispatch({ type: 'ui/set-theme', payload: newTheme });
  }

  const icon = theme == Theme.Dark ? '🌑'
    : theme == Theme.Light ? '🔆' : '🌓';
  useEffect(() => {
    switch (theme) {
      case Theme.Light:
        document.documentElement.setAttribute('data-theme', 'light');
        break;
      case Theme.Dark:
        document.documentElement.setAttribute('data-theme', 'dark');
        break;
      case Theme.Automatic:
        document.documentElement.removeAttribute('data-theme');
        break;
    }
  });
  return (
    <div className="settings-container">
      Theme:
      <button className="ui-theme-toggle" onClick={toggleTheme}>{icon}</button>
    </div>
  );
};
