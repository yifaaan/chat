CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    fullname VARCHAR(64) NOT NULL
    email VARCHAR(64) NOT NULL,
    -- hashed argon2
    password VARCHAR(64) NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TYPE chat_type AS ENUM ('sigle', 'group', 'private_channel', 'public_channel')

CREATE TABLE IF NOT EXISTS chats (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    -- user id list
    members BIGINT[] NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAl PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    sender_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (chat_id) REFERENCES chats(id),
    FOREIGN KEY (sender_id) REFERENCES users(id)
);

-- index
CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages(chat_id, create_at DESC);

CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id);