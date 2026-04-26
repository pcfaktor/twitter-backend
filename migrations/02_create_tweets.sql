CREATE TABLE IF NOT EXISTS tweets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR(280) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO
    tweets (author_id, content, created_at)
VALUES
    (
        '00000000-0000-0000-0000-000000000001',
        'Wazzup',
        now() - interval '5 minutes'
    ),
    (
        '00000000-0000-0000-0000-000000000002',
        'WAO',
        now() - interval '10 minutes'
    ),
    (
        '00000000-0000-0000-0000-000000000003',
        'Just chilling...',
        now() - interval '15 minutes'
    ),
    (
        '00000000-0000-0000-0000-000000000004',
        'GM',
        now() - interval '20 minutes'
    ),
    (
        '00000000-0000-0000-0000-000000000005',
        'Let''s build',
        now() - interval '25 minutes'
    );