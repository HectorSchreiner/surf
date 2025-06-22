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
    "references" JSONB NOT NULL,

    UNIQUE (key)
);

-- We create an index for efficiently searching the vulnerabilities
CREATE INDEX vulnerabilities_search ON vulnerabilities USING GIN ((to_tsvector('english', key) || to_tsvector('english', name) || to_tsvector('english', description)));