DROP TABLE IF EXISTS storytag;
DROP TABLE IF EXISTS gametag;
DROP TABLE IF EXISTS tooltag;

CREATE TABLE IF NOT EXISTS producttag
(
    pid  INTEGER NOT NULL,
    tid INTEGER NOT NULL,
    FOREIGN KEY (pid) REFERENCES product(id),
    FOREIGN KEY (tid) REFERENCES tag(id),
    PRIMARY KEY (pid, tid)
);

