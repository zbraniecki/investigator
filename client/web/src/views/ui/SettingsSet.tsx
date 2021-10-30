import React from 'react';
import Button from '@material-ui/core/Button';
import Checkbox from '@material-ui/core/Checkbox';
import BrightnessAutoIcon from '@material-ui/icons/BrightnessAuto';
import BrightnessLowIcon from '@material-ui/icons/BrightnessLow';
import BrightnessHighIcon from '@material-ui/icons/BrightnessHigh';
import {
  Theme,
} from '../../store/ui';

export default ({ storedTheme, storedDisplayValues, onThemeChange, onDisplayValuesChange }: Props) => {
  let themeIcon;

  if (storedTheme === Theme.Light) {
    themeIcon = <BrightnessHighIcon fontSize="large" />;
  } else if (storedTheme === Theme.Dark) {
    themeIcon = <BrightnessLowIcon fontSize="large" />;
  } else {
    themeIcon = <BrightnessAutoIcon fontSize="large" />;
  }

  let displayValuesIcon = <Checkbox checked={storedDisplayValues}
  color="white"
    onClick={onDisplayValuesChange} />;

  return (
  <>
    {displayValuesIcon}
    <Button onClick={onThemeChange}>
      {themeIcon}
    </Button>
  </>
  );
};
