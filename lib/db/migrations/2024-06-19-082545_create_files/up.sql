-- Your SQL goes here
CREATE TABLE files (
    message_id UUID PRIMARY KEY REFERENCES messages(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL
);