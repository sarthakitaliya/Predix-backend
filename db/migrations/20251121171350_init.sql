CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE share_type AS ENUM ('yes', 'no');
CREATE TYPE order_status AS ENUM ('partially_filled', 'filled', 'canceled');
CREATE TYPE market_status AS ENUM ('open', 'closed', 'resolved');
CREATE TYPE market_outcome AS ENUM ('yes', 'no', 'not_decided');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    solana_address VARCHAR(44) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE markets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    market_id TEXT UNIQUE NOT NULL,
    market_pda TEXT UNIQUE NOT NULL,
    metadata_url TEXT NOT NULL,

    yes_mint TEXT UNIQUE NOT NULL,
    no_mint TEXT UNIQUE NOT NULL,
    usdc_vault TEXT UNIQUE NOT NULL,
    status market_status DEFAULT 'open',
    outcome market_outcome DEFAULT 'not_decided',
    close_time TIMESTAMP WITH TIME ZONE NOT NULL,
    resolve_time TIMESTAMP WITH TIME ZONE,

    title TEXT NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    image_url TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_markets_status ON markets (status);
CREATE INDEX idx_markets_category ON markets (category);
CREATE INDEX idx_markets_close_time ON markets (close_time);
