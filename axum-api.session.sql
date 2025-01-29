INSERT INTO accounts (user_id, balance_cents)
VALUES (
        (SELECT id
        FROM users
        WHERE username = 'alice'),
            10000
    );
