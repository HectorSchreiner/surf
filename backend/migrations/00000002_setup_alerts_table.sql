CREATE TABLE alerts (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    name TEXT NOT NULL,
    message TEXT NOT NULL,
    severity TEXT NOT NULL
)