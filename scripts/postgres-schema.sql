CREATE TABLE tokens (
    id              SERIAL      NOT NULL PRIMARY KEY, 
    token           CHAR(45)    NOT NULL UNIQUE, -- this is hashed, then converted to base64 so it takes more space
    sensor_uid      CHAR(36)    NOT NULL,
    firebase_uid    CHAR(28)    NOT NULL,
    created         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);