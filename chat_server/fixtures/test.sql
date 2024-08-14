-- insert workspace
INSERT INTO workspace (name, owner_id)
VALUES ('acme', '0'),
    ('foo', 0),
    ('bar', 0);
-- insert users
INSERT INTO users (ws_id, email, fullname, password_hash)
VALUES (
        1,
        'maya@acme.com',
        'maya acme',
        '$argon2id$v=19$m=19456,t=2,p=1$FUXMnKyerjElJpX0WduyWw$YKU5sPf2jvWY2p6tapH5M6nc2d8uSawMZn3pR9UBAuc'
    ),
    (
        1,
        'alice@acme.com',
        'alice acme',
        '$argon2id$v=19$m=19456,t=2,p=1$FUXMnKyerjElJpX0WduyWw$YKU5sPf2jvWY2p6tapH5M6nc2d8uSawMZn3pR9UBAuc'
    ),
    (
        1,
        'bob@acme.com',
        'bob acme',
        '$argon2id$v=19$m=19456,t=2,p=1$FUXMnKyerjElJpX0WduyWw$YKU5sPf2jvWY2p6tapH5M6nc2d8uSawMZn3pR9UBAuc'
    ),
    (
        1,
        'charlie@acme.com',
        'charlie acme',
        '$argon2id$v=19$m=19456,t=2,p=1$FUXMnKyerjElJpX0WduyWw$YKU5sPf2jvWY2p6tapH5M6nc2d8uSawMZn3pR9UBAuc'
    ),
    (
        1,
        'daisy@acme.com',
        'daisy acme',
        '$argon2id$v=19$m=19456,t=2,p=1$FUXMnKyerjElJpX0WduyWw$YKU5sPf2jvWY2p6tapH5M6nc2d8uSawMZn3pR9UBAuc'
    );
-- insert 4 chats
-- insert public/private channel
INSERT INTO chats(ws_id, name, type, members)
VALUES (1, 'general', 'public_channel', '{1,2,3,4,5}'),
    (1, 'private', 'private_channel', '{1,2,3}');
-- insert unnamed chat
INSERT INTO chats(
        ws_id,
        type,
        members
    )
VALUES (1, 'single', '{1,2}'),
    (1, 'group', '{1,3,4}');
