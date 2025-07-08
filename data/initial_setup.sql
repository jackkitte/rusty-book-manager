INSERT INTO
    roles (name)
VALUES ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (
        name,
        email,
        password_hash,
        role_id
    )
SELECT 'kakeru.tamashiro', 'kakeru.tamashiro@example.com', '$2b$12$Wk5LQTTmz2VvEaBRp5ZhNu33U2pGesHfXOgSWyanLdEc8Bsvjp59K', role_id
FROM roles
WHERE
    name LIKE 'Admin';