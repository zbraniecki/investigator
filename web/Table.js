import React from "react";

import { useTable, useExpanded } from "react-table";

const EXPANDER = "EXPANDER";

const Table = ({ columns, data: tableData, renderSubComponent }) => {
  const expanderCol = React.useMemo(
    () => ({
      Cell: ({ row }) => (
        <div {...row.getToggleRowExpandedProps()}>
          {row.isExpanded ? "👇" : "👈"}
        </div>
      ),
      Header: ({ toggleAllRowsExpanded }) => (
        <span onClick={() => toggleAllRowsExpanded(false)}>Minimize all</span>
      ),
      id: EXPANDER
    }),
    []
  );

  const tableColumns = React.useMemo(() => {
    const headers = [...columns, ...(renderSubComponent ? [expanderCol] : [])];

    return headers;
  }, [columns, renderSubComponent, expanderCol]);

  const data = React.useMemo(() => tableData, [tableData]);

  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    rows,
    prepareRow,
    flatHeaders
  } = useTable(
    {
      columns: tableColumns,
      data
    },
    useExpanded
  );

  const renderTableHead = headerGroup => (
    <tr {...headerGroup.getHeaderGroupProps()}>
      {headerGroup.headers.map(column => (
        <th {...column.getHeaderProps()}>{column.render("Header")}</th>
      ))}
    </tr>
  );

  const renderTableBody = row => {
    prepareRow(row);
    return (
      <React.Fragment key={row.index}>
        <tr {...row.getRowProps()}>
          {row.cells.map(cell => (
            <td {...cell.getCellProps()}>{cell.render("Cell")}</td>
          ))}
        </tr>
        {renderSubComponent && row.isExpanded && (
          <tr>
            <td colSpan={flatHeaders.length}>{renderSubComponent(row)}</td>
          </tr>
        )}
      </React.Fragment>
    );
  };
  return (
    <table {...getTableProps()}>
      <thead>{headerGroups.map(renderTableHead)}</thead>
      <tbody {...getTableBodyProps()}>{rows.map(renderTableBody)}</tbody>
    </table>
  );
};

export default Table;

