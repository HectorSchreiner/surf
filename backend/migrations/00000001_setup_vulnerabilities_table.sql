CREATE TABLE vulnerabilities (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,

    key TEXT NOT NULL,
    reserved_at TIMESTAMPTZ,
    published_at TIMESTAMPTZ,
    rejected_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    UNIQUE (key)
)