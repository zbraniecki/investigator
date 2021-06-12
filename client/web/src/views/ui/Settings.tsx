import React, { useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import {
  Theme,
  getTheme,
} from '../../store/ui';

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
      default:
        break;
    }
    dispatch({ type: 'ui/set-theme', payload: newTheme });
  }

  let icon;
  switch (theme) {
    case Theme.Automatic:
      icon = '🔆';
      break;
    case Theme.Light:
      icon = '🌓';
      break;
    case Theme.Dark:
      icon = '🌑';
      break;
    default:
      break;
  }

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
      default:
        break;
    }
  });
  return (
    <div className="settings-container">
      Theme:
      <button type="button" className="ui-theme-toggle" onClick={toggleTheme}>{icon}</button>
    </div>
  );
};
