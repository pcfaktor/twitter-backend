CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    bio TEXT NOT NULL DEFAULT '',
    avatar_url TEXT,
    banner_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO
    users (
        id,
        username,
        display_name,
        avatar_url,
        banner_url
    )
VALUES
    (
        '00000000-0000-0000-0000-000000000001',
        'falcon',
        'Edward Falcon',
        'https://static.wikia.nocookie.net/capcomdatabase/images/0/06/Falcon.png/revision/latest/top-crop/width/200/height/150?cb=20260217055012',
        'https://picsum.photos/seed/falcon/600/200'
    ),
    (
        '00000000-0000-0000-0000-000000000002',
        'rouge',
        'Rouge',
        'https://static.wikia.nocookie.net/capcomdatabase/images/f/f4/PSRouge2.png/revision/latest/top-crop/width/200/height/150?cb=20260217071123',
        'https://picsum.photos/seed/rouge/600/200'
    ),
    (
        '00000000-0000-0000-0000-000000000003',
        'gunrock',
        'Gunrock',
        'https://static.wikia.nocookie.net/capcomdatabase/images/0/05/Gunrock3.PNG/revision/latest/top-crop/width/200/height/150?cb=20260217064539',
        'https://picsum.photos/seed/gunrock/600/200'
    ),
    (
        '00000000-0000-0000-0000-000000000004',
        'ayame',
        'Ayame',
        'https://static.wikia.nocookie.net/capcomdatabase/images/f/f4/PS_Ayame.png/revision/latest/top-crop/width/200/height/150?cb=20260217054444',
        'https://picsum.photos/seed/ayame/600/200'
    ),
    (
        '00000000-0000-0000-0000-000000000005',
        'jack',
        'Jack',
        'https://static.wikia.nocookie.net/capcomdatabase/images/b/bf/PSJack2.png/revision/latest/top-crop/width/200/height/150?cb=20260217064855',
        'https://picsum.photos/seed/jack/600/200'
    );