-- Your SQL goes here
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    group_id UUID REFERENCES groups(id) ON DELETE SET NULL,
    content TEXT,
    type message_types,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);