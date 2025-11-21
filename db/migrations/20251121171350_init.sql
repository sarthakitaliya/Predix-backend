CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE share_type AS ENUM ('yes', 'no');
CREATE TYPE order_status AS ENUM ('partially_filled', 'filled', 'canceled');
CREATE TYPE market_status AS ENUM ('open', 'closed', 'resolved');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    solana_address VARCHAR(44) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE markets (
    id UUID PRIMARY KEY,
    title VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    status market_status DEFAULT 'open',
    closed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE closed_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    market_id UUID NOT NULL REFERENCES markets(id),
    type share_type NOT NULL,
    price NUMERIC NOT NULL,
    qty NUMERIC NOT NULL,
    filled_qty NUMERIC NOT NULL,  
    status order_status NOT NULL,
    closed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);