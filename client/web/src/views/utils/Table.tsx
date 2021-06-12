import React from 'react';
import Paper from '@material-ui/core/Paper';
import { makeStyles, createStyles } from '@material-ui/core/styles';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Typography from '@material-ui/core/Typography';

const useStyles = makeStyles(() => createStyles({
  root: {
    flexGrow: 1,
    height: '100vh',
  },
  tableContainer: {
    backgroundColor: 'inherit',
  },
}));

interface Props {
  data: any,
  style: any,
}

export default (props: Props) => {
  const classes = useStyles();

  const { data, style } = props;

  function renderTableHead() {
    if (style.header === null) {
      return '';
    }

    return (
      <TableHead>
        <TableRow>
          <TableCell>Symbol</TableCell>
          <TableCell align="right">Symbol</TableCell>
          <TableCell align="right">Price</TableCell>
        </TableRow>
      </TableHead>
    );
  }

  return (
    <TableContainer component={Paper} className={classes.tableContainer}>
      <Table className={classes.table} aria-label="simple table">
        { renderTableHead() }
        <TableBody>
          {data.map((row) => (
            <TableRow key={row.key}>
              <TableCell component="th" scope="row">
                {row.symbol}
              </TableCell>
              <TableCell align="right">
                <Typography>30000</Typography>
                <Typography>-4.3%</Typography>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
