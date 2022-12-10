CREATE TABLE tokens (
    id              SERIAL      NOT NULL PRIMARY KEY, 
    token           CHAR(36)    NOT NULL UNIQUE,
    firebase_uid    CHAR(28)    NOT NULL,
    created         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO tokens (token, firebase_uid) VALUES ('97345bc3-a924-491e-b17b-11bda8b70901', '75OK7rgl3YRCDNrypdh5dyJ6lcx2');
SELECT * FROM tokens;