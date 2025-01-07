BEGIN;

CREATE TABLE IF NOT EXISTS education_user
(
    id          SERIAL,
    uuid        UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    first_name  TEXT                      DEFAULT '',
    second_name TEXT                      DEFAULT '',
    last_name   TEXT                      DEFAULT '',
    phone       TEXT                      DEFAULT '',
    birth_date  DATE,
    created_at  TIMESTAMP        NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP        NOT NULL DEFAULT NOW()
);

COMMIT;