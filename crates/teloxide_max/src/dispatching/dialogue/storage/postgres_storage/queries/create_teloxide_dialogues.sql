CREATE TABLE IF NOT EXISTS teloxide_max_dialogues (
    chat_id BIGINT PRIMARY KEY,
    dialogue BYTEA NOT NULL
)