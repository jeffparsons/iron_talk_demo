#!/bin/bash -e

db_file=/tmp/iron_talk_demo.sqlite

rm $db_file

echo "Creating submission table..."
sqlite3 $db_file <<-EOS
    CREATE TABLE submission (
        id VARCHAR(255) PRIMARY KEY NOT NULL,
        url TEXT NOT NULL
    );
EOS

echo "Creating comment table..."
sqlite3 $db_file <<-EOS
    CREATE TABLE comment (
        id VARCHAR(255) PRIMARY KEY NOT NULL,
        parent_id VARCHAR(255) NOT NULL,
        comment_plaintext TEXT NOT NULL
    );
EOS

echo "Success!"
