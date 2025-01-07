BEGIN;

CREATE TYPE event_type_enum AS ENUM ('meet', 'webinar', 'announcement');

CREATE TABLE IF NOT EXISTS event
(
    id                    SERIAL,
    uuid                  UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    title                 TEXT             NOT NULL DEFAULT '',
    description           TEXT                      DEFAULT '',
    event_type            event_type_enum  NOT NULL DEFAULT 'meet',
    date                  TIMESTAMP,
    icon                  TEXT,
    background_image      TEXT,
    background_fill_color TEXT,
    background_color      TEXT,
    text_color            TEXT,
    created_at            TIMESTAMP        NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMP        NOT NULL DEFAULT NOW()
);

COMMIT;