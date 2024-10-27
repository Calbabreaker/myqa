CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    reputation INT NOT NULL
);
