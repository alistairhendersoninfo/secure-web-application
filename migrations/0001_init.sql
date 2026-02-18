-- roles and users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username CITEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY,
    name CITEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- agents
CREATE TABLE IF NOT EXISTS agents (
    id UUID PRIMARY KEY,
    hostname TEXT NOT NULL,
    os TEXT NOT NULL,
    version TEXT NOT NULL,
    enrolled_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen TIMESTAMPTZ
);

-- plugins and versions
CREATE TABLE IF NOT EXISTS plugins (
    id UUID PRIMARY KEY,
    name CITEXT UNIQUE NOT NULL,
    category TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plugin_versions (
    id UUID PRIMARY KEY,
    plugin_id UUID NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    version TEXT NOT NULL,
    wasm_sha256 BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (plugin_id, version)
);

-- plugin configurations (declarative desired state)
CREATE TABLE IF NOT EXISTS plugin_configs (
    id UUID PRIMARY KEY,
    plugin_id UUID NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    schema_version TEXT NOT NULL,
    config JSONB NOT NULL,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- deployments (desired state assignment to agents or groups)
CREATE TABLE IF NOT EXISTS deployments (
    id UUID PRIMARY KEY,
    plugin_config_id UUID NOT NULL REFERENCES plugin_configs(id) ON DELETE CASCADE,
    target TEXT NOT NULL, -- could be agent id, group name, label selector
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- audit log with hash chaining
CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL PRIMARY KEY,
    ts TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor UUID,
    action TEXT NOT NULL,
    details JSONB,
    prev_hash BYTEA,
    this_hash BYTEA NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_ts ON audit_log (ts);

-- RLS placeholders (enable and policies to be added as needed)
-- ALTER TABLE users ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
