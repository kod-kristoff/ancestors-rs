-- Your SQL goes here
CREATE TABLE agents (
    id VARCHAR(26) NOT NULL PRIMARY KEY,
    body TEXT NOT NULL,
    updated TIMESTAMP NOT NULL,
    updated_by VARCHAR(255) NOT NULL
)
