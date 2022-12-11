CREATE TABLE tokens (
    id              SERIAL      NOT NULL PRIMARY KEY, 
    token           CHAR(36)    NOT NULL UNIQUE,
    firebase_uid    CHAR(28)    NOT NULL,
    created         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);