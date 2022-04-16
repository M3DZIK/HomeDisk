#!/bin/bash

if [ -f 'homedisk.db' ]
then
    echo 'Database `homedisk.db` exists!'
    exit 1
fi

# create a database using the commands from the `tables.sql` file
sqlite3 homedisk.db < tables.sql

echo 'Created SQLite database `homedisk.db`!'
