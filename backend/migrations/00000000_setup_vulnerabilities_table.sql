CREATE TABLE vulnerabilities (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    key TEXT NOT NULL,

    UNIQUE (key)
)