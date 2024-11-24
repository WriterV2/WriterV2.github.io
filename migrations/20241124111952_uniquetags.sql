DELETE FROM producttag;
CREATE TABLE IF NOT EXISTS newtag
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL UNIQUE
);

WITH uniquetags AS (
    SELECT id, name, ROW_NUMBER() OVER (PARTITION BY name ORDER BY id) AS row_num
    FROM tag
)

INSERT INTO newtag (id, name)
SELECT id, name
FROM uniquetags
WHERE row_num = 1;

DROP TABLE tag;
ALTER TABLE newtag RENAME TO tag;
