import {
  Theme,
  getTheme,
} from '../store/ui';
import { useSelector, useDispatch } from 'react-redux'

export default () => {
  const theme = useSelector(getTheme);
 
  let icon = theme == Theme.Dark ? "dark" :
    theme == Theme.Light ? "light" : "automatic";
  return (
    <div>
      Theme:
      <button>{icon}</button>
    </div>
  );
}
