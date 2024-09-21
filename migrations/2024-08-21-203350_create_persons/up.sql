CREATE TABLE persons (
    id VARCHAR(26) NOT NULL PRIMARY KEY,
    extracted BOOL NOT NULL,
    body TEXT NOT NULL,
    updated TIMESTAMP NOT NULL,
    updated_by VARCHAR(255) NOT NULL
)
