-- Your SQL goes here
CREATE TABLE forwarded_messages (
    id UUID PRIMARY KEY,
    original_message_id UUID REFERENCES messages(id) ON DELETE CASCADE,
    new_message_id UUID REFERENCES messages(id) ON DELETE CASCADE,
    forwarded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);