-- Add up migration script here

-- create spl_token table in sqlite3
CREATE TABLE spl_token (
    mint TEXT PRIMARY KEY, -- mint address
    smart_address TEXT NOT NULL, -- smart address, who related to this token
    monitor_status TEXT NOT NULL DEFAULT 'active', -- monitor status
    strategy_name TEXT NOT NULL, -- strategy name
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- created at
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- updated at
);