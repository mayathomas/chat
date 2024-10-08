-- insert workspace
INSERT INTO workspace (name, owner_id)
VALUES ('acme', '0'),
    ('foo', 0),
    ('bar', 0);
-- insert users
INSERT INTO users (ws_id, email, fullname, password_hash)
VALUES (
        1,
        'maya@qq.com',
        'maya thomas',
        '$argon2id$v=19$m=19456,t=2,p=1$f6NhdwtywoY/8TTM1VcOJA$+IElCgu1wL31bEz+hA7Hg1vf7qLR3YL10vqD8Paw9Vo'
    ),
    (
        1,
        'alice@acme.com',
        'alice acme',
        '$argon2id$v=19$m=19456,t=2,p=1$f6NhdwtywoY/8TTM1VcOJA$+IElCgu1wL31bEz+hA7Hg1vf7qLR3YL10vqD8Paw9Vo'
    ),
    (
        1,
        'bob@acme.com',
        'bob acme',
        '$argon2id$v=19$m=19456,t=2,p=1$f6NhdwtywoY/8TTM1VcOJA$+IElCgu1wL31bEz+hA7Hg1vf7qLR3YL10vqD8Paw9Vo'
    ),
    (
        1,
        'charlie@acme.com',
        'charlie acme',
        '$argon2id$v=19$m=19456,t=2,p=1$f6NhdwtywoY/8TTM1VcOJA$+IElCgu1wL31bEz+hA7Hg1vf7qLR3YL10vqD8Paw9Vo'
    ),
    (
        1,
        'daisy@acme.com',
        'daisy acme',
        '$argon2id$v=19$m=19456,t=2,p=1$f6NhdwtywoY/8TTM1VcOJA$+IElCgu1wL31bEz+hA7Hg1vf7qLR3YL10vqD8Paw9Vo'
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
-- insert messages
INSERT INTO messages(chat_id, sender_id, content)
VALUES (1, 1, 'Hello, world!'),
    (1, 2, 'Hi, there!'),
    (1, 3, 'How are you?'),
    (1, 4, 'I am fine, thank you!'),
    (1, 5, 'Good to hear that!'),
    (1, 1, 'Hello, world!'),
    (1, 2, 'Hi, there!'),
    (1, 3, 'How are you?'),
    (1, 1, 'Hello, world!'),
    (1, 1, 'Hello, world!');
