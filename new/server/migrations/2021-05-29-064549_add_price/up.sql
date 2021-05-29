CREATE TABLE prices (
  base VARCHAR  NOT NULL,
  target VARCHAR  NOT NULL,
  ts TIMESTAMP NOT NULL,
  value DOUBLE NOT NULL,
  PRIMARY KEY (base, target),
  FOREIGN KEY (base) REFERENCES coins (id),
  FOREIGN KEY (target) REFERENCES coins (id)
)
