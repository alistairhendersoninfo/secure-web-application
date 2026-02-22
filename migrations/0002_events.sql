-- events / logs storage (minimal v0)
CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL PRIMARY KEY,
    ts TIMESTAMPTZ NOT NULL DEFAULT now(),
    agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    source TEXT NOT NULL,           -- e.g., journald, apparmor, falco
    level TEXT NOT NULL,            -- info|warn|error|debug
    message TEXT NOT NULL,
    details JSONB,                  -- parsed structured fields
    seq BIGINT,                     -- optional per-agent sequence
    signature BYTEA                 -- optional Ed25519 signature
);

CREATE INDEX IF NOT EXISTS idx_events_ts ON events (ts);
CREATE INDEX IF NOT EXISTS idx_events_agent ON events (agent_id);
CREATE INDEX IF NOT EXISTS idx_events_source ON events (source);
CREATE INDEX IF NOT EXISTS idx_events_level ON events (level);
CREATE INDEX IF NOT EXISTS idx_events_details_gin ON events USING GIN (details);
