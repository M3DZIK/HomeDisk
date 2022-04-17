CREATE TABLE user (
    id              TEXT PRIMARY KEY UNIQUE,
    username        TEXT NOT NULL UNIQUE,
    password        TEXT NOT NULL
);
