CREATE TABLE IF NOT EXISTS chat_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    updated_at DATETIME,
    created_at DATETIME
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER,
    role TEXT(50) NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME,
    FOREIGN KEY (session_id) REFERENCES chat_sessions (id)
);