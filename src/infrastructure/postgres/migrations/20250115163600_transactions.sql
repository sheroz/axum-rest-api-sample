-- create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    balance_cents bigint NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
-- create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    source_account_id UUID NOT NULL,
    destination_account_id UUID NOT NULL,
    amount_cents bigint NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
-- add users with customer role
INSERT INTO users (
        username,
        email,
        password_hash,
        password_salt,
        active,
        roles,
        created_at,
        updated_at
    )
VALUES (
        'alice',
        'alice@mail.com',
        -- password: pswd1234, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51',
        'pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF',
        'true',
        'customer',
        now(),
        now()
    );
INSERT INTO users (
        username,
        email,
        password_hash,
        password_salt,
        active,
        roles,
        created_at,
        updated_at
    )
VALUES (
        'bob',
        'bob@mail.com',
        -- password: pswd1234, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51',
        'pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF',
        'true',
        'customer',
        now(),
        now()
    );
-- populate accounts table with initial balance
INSERT INTO accounts (user_id, balance_cents)
VALUES (
        (
            SELECT id
            FROM users
            WHERE username = 'alice'
        ),
        10000
    );
INSERT INTO accounts (user_id, balance_cents)
VALUES (
        (
            SELECT id
            FROM users
            WHERE username = 'bob'
        ),
        10000
    );