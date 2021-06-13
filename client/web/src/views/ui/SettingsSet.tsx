import React from 'react';
import Button from '@material-ui/core/Button';
import BrightnessAutoIcon from '@material-ui/icons/BrightnessAuto';
import BrightnessLowIcon from '@material-ui/icons/BrightnessLow';
import BrightnessHighIcon from '@material-ui/icons/BrightnessHigh';
import {
  Theme,
} from '../../store/ui';

export default ({ storedTheme, onThemeChange }: Props) => {
  let themeIcon;
  if (storedTheme === Theme.Light) {
    themeIcon = <BrightnessHighIcon fontSize="large" />;
  } else if (storedTheme === Theme.Dark) {
    themeIcon = <BrightnessLowIcon fontSize="large" />;
  } else {
    themeIcon = <BrightnessAutoIcon fontSize="large" />;
  }

  return (
    <Button onClick={onThemeChange}>
      {themeIcon}
    </Button>
  );
};
