CREATE TABLE IF NOT EXISTS home_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    fingerprint TEXT NOT NULL UNIQUE,
    theme_id TEXT NOT NULL DEFAULT 'late',
    timezone TEXT NOT NULL DEFAULT 'UTC',
    bio TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS home_chat_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS home_chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES home_chat_rooms(id),
    user_id UUID NOT NULL REFERENCES home_users(id),
    body TEXT NOT NULL,
    reply_to_id UUID REFERENCES home_chat_messages(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS home_cat_trees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES home_users(id),
    growth_points INT NOT NULL DEFAULT 0,
    last_cared_at TIMESTAMPTZ,
    seed BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO home_chat_rooms (name, description) VALUES
    ('general', 'General chat'),
    ('tech', 'Tech talk'),
    ('music', 'Music and radio'),
    ('random', 'Anything goes')
ON CONFLICT DO NOTHING;
