import React from 'react';
import Paper from '@material-ui/core/Paper';
import { makeStyles, createStyles, Theme } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

const useStyles = makeStyles((theme: Theme) => createStyles({
  root: {
    flexGrow: 1,
    height: '100vh',
  },
}));

function createData(symbol: string, price: number, pricePerc24h: number, mcapPerc24h: number) {
  return {
    symbol, price, pricePerc24h, mcapPerc24h,
  };
}

export default (props) => {
  const classes = useStyles();

  const { kind } = props;
  let rows;
  if (kind == 'markets') {
    rows = [
      createData('BTC', 36600, 0.24, 0.25),
      createData('ETH', 2650, 0.12, 0.09),
    ];
  } else {
    rows = [
      createData('TSLA', 36600, 0.24, 0.25),
      createData('GOOG', 2650, 0.12, 0.09),
    ];
  }

  return (
    <TableContainer component={Paper}>
      <Table className={classes.table} aria-label="simple table">
        <TableHead>
          <TableRow>
            <TableCell>Symbol</TableCell>
            <TableCell align="right">Price</TableCell>
            <TableCell align="right">Price % 24h</TableCell>
            <TableCell align="right">Mcap % 24h</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {rows.map((row) => (
            <TableRow key={row.name}>
              <TableCell component="th" scope="row">
                {row.symbol}
              </TableCell>
              <TableCell align="right">{row.price}</TableCell>
              <TableCell align="right">{row.pricePerc24h}</TableCell>
              <TableCell align="right">{row.mcapPerc24h}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
