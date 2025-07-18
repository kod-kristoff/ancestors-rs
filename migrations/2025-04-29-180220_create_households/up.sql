CREATE TABLE households (
    id VARCHAR(26) NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    body TEXT NOT NULL,
    updated TIMESTAMP NOT NULL,
    updated_by VARCHAR(255) NOT NULL
);

CREATE TABLE household_members (
    household_id VARCHAR(26) NOT NULL,
    person_id VARCHAR(26) NOT NULL,
    role TEXT,
    from_date TEXT,
    to_date TEXT,
    CONSTRAINT household_members_pk PRIMARY KEY (household_id, person_id),
    CONSTRAINT household_members_household_fk FOREIGN KEY (household_id) references households (id),
    CONSTRAINT household_members_person_fk FOREIGN KEY (person_id) references persons (id)
)
