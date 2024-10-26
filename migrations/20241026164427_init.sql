CREATE TABLE IF NOT EXISTS product
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL UNIQUE,
    description TEXT                NOT NULL UNIQUE,
    uploaddate  INTEGER             NOT NULL,
    updatedate  INTEGER             NOT NULL
);

CREATE TABLE IF NOT EXISTS story
(
    id       INTEGER PRIMARY KEY NOT NULL,
    language TEXT                NOT NULL,
    pdf      BLOB                NOT NULL,
    epub     BLOB                NOT NULL,
    pid      INTEGER             NOT NULL,
    FOREIGN KEY (pid) REFERENCES product(id)
);

CREATE TABLE IF NOT EXISTS tool
(
    id       INTEGER PRIMARY KEY NOT NULL,
    repolink TEXT                NOT NULL,
    pid      INTEGER             NOT NULL,
    FOREIGN KEY (pid) REFERENCES product(id)
);

CREATE TABLE IF NOT EXISTS game
(
    id       INTEGER PRIMARY KEY NOT NULL,
    repolink TEXT                NOT NULL,
    pid      INTEGER             NOT NULL,
    FOREIGN KEY (pid) REFERENCES product(id)
);

CREATE TABLE IF NOT EXISTS tag
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS storytag
(
    sid  INTEGER NOT NULL,
    tgid INTEGER NOT NULL,
    FOREIGN KEY (sid) REFERENCES story(id),
    FOREIGN KEY (tgid) REFERENCES tag(id),
    PRIMARY KEY (sid, tgid)
);

CREATE TABLE IF NOT EXISTS tooltag
(
    tid  INTEGER NOT NULL,
    tgid INTEGER NOT NULL,
    FOREIGN KEY (tid) REFERENCES tool(id),
    FOREIGN KEY (tgid) REFERENCES tag(id),
    PRIMARY KEY (tid, tgid)
);

CREATE TABLE IF NOT EXISTS gametag
(
    gid  INTEGER NOT NULL,
    tgid INTEGER NOT NULL,
    FOREIGN KEY (gid) REFERENCES game(id),
    FOREIGN KEY (tgid) REFERENCES tag(id),
    PRIMARY KEY (gid, tgid)
);
