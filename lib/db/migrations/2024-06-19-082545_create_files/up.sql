-- Your SQL goes here
CREATE TABLE files (
    message_id UUID PRIMARY KEY REFERENCES messages(id) ON DELETE CASCADE,
    fileName VARCHAR(255) NOT NULL
);