CREATE TABLE user (
    id              INTEGER PRIMARY KEY,
    username        TEXT NOT NULL,
    password        TEXT NOT NULL
);

/* create root user with password 'root' */
INSERT INTO user (username, password) VALUES ('root', '99adc231b045331e514a516b4b7680f588e3823213abe901738bc3ad67b2f6fcb3c64efb93d18002588d3ccc1a49efbae1ce20cb43df36b38651f11fa75678e8');
