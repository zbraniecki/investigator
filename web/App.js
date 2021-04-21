import React from "react";

import Table from "./Table";

// import "./styles.css";

const data = [
  { a: "foo", b: "bar", c: "baz" },
  { a: "foo", b: "bar", c: "baz" },
  { a: "foo", b: "bar", c: "baz" },
  { a: "foo", b: "bar", c: "baz" },
  { a: "foo", b: "bar", c: "baz" }
];

export default function App() {
  const columns = [
    {
      Header: "First name",
      accessor: "a"
    },
    {
      Header: "Last name",
      accessor: "b"
    },
    {
      Header: "Country",
      accessor: "c"
    }
  ];

  return (
    <div className="App">
      <Table
        data={data}
        columns={columns}
        renderSubComponent={row => "Expanded row"}
      />
    </div>
  );
}
