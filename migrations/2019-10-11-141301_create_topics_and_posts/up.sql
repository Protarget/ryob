CREATE TABLE topics (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    created_by BIGINT REFERENCES users(id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    posted_in BIGINT REFERENCES topics(id) NOT NULL,
    created_by BIGINT REFERENCES users(id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    content TEXT NOT NULL
);