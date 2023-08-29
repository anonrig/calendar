CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE providers AS ENUM('google');
CREATE TYPE roles AS ENUM('owner', 'member');

CREATE TABLE users (
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    provider providers NOT NULL,
    provider_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE groups (
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_groups (
   id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
   role roles NOT NULL DEFAULT 'member',
   user_id uuid NOT NULL REFERENCES users(id),
   group_id uuid NOT NULL REFERENCES groups(id),
   created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

--Ensure that there is only 1 owner of a user_group
CREATE UNIQUE INDEX user_groups_owner_idx ON user_groups(group_id, role) WHERE role = 'owner';
