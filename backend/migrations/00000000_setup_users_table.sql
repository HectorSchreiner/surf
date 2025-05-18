CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    name TEXT NOT NULL,
    reset BOOLEAN NOT NULL,

    UNIQUE(email)
)