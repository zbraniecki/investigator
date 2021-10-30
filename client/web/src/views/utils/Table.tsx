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
import Collapse from '@material-ui/core/Collapse';
import KeyboardArrowDownIcon from '@material-ui/icons/KeyboardArrowDown';
import KeyboardArrowUpIcon from '@material-ui/icons/KeyboardArrowUp';
import IconButton from '@material-ui/core/IconButton';

const useStyles = makeStyles(() => createStyles({
  root: {
    flexGrow: 1,
    height: '100vh',
  },
  tableContainer: {
    backgroundColor: 'inherit',
  },
}));

const useRowStyles = makeStyles({
  root: {
    '& > *': {
    },
  },
  rootWithSubs: {
    '& > *': {
      borderBottom: 'unset',
    },
  },
});

interface Props {
  data: any,
  style: any,
}

function getCellValue(colValue, row, cellKey) {
  if (typeof colValue.key === 'string') {
    return row[colValue.key];
  }
  return colValue.key.map((key, idx) => {
    let value = typeof key === 'string' ? row[key] : row[key.key];
    let k = `${cellKey}-${idx}`;
    if (typeof key !== 'string') {
      return <Typography key={k} style={{ color: row[key.color] }}>{value}</Typography>;
    } else {
      return <Typography key={k}>{value}</Typography>;
    }
  });
  return 'foo';
}

function Row(props: { row: ReturnType<typeof createData> }) {
  const { row, style } = props;
  const [open, setOpen] = React.useState(false);
  const classes = useRowStyles();

  return (
    <>
      <TableRow className={row.sub ? classes.rootWithSubs : classes.root}>
        { style.nested ?
          <TableCell width="10%">
            { row.sub
              ? (
                <IconButton size="small" onClick={() => setOpen(!open)}>
                  {open ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
                </IconButton>
              )
              : <></>}
          </TableCell>
          : <></>}
        { style.columns.map((col, idx) => {
          let key = `row.key-${col.label}`;
          let value = getCellValue(col.value, row, key);
          return <TableCell component="th" scope="row" align={col.align} key={key}>
            {value}
          </TableCell>;
        })}
      </TableRow>
      { row.sub
        ? (
          <TableRow>
            <TableCell style={{ padding: 0 }} colSpan={6}>
              <Collapse in={open} timeout="auto" unmountOnExit>
                <Paper>
                  <Table>
                    <TableBody>
                      {row.sub.map((subRow) => (
                        <TableRow key={subRow.key}>
                          <TableCell width="10%" />
                          <TableCell component="th" scope="row">
                            {subRow.symbol}
                          </TableCell>
                          <TableCell align="right">
                            <Typography>{subRow.value}</Typography>
                            <Typography style={{ color: subRow.color }}>{subRow.change}</Typography>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </Paper>
              </Collapse>
            </TableCell>
          </TableRow>
        )
        : <></>}
    </>
  );
}

export default (props: Props) => {
  const classes = useStyles();

  const { data, style } = props;

  function renderTableHead() {
    if (!style.header) {
      return <></>;
    }

    let nestedCell = <></>;
    if (style.nested) {
      nestedCell = <TableCell align="left"></TableCell>;
    }

    return (
      <TableHead>
        <TableRow>
          { nestedCell }
          {style.columns.map((col, idx) => (
            <TableCell key={col.label} align={col.align}>{col.label}</TableCell>
          ))}
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
            <Row key={row.key} row={row} style={style} />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
