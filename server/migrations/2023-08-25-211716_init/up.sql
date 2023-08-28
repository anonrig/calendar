-- TODO: binary(16) is more efficient instead of varchar(32) for fields with UUID, it can be updated for future optimization
CREATE TABLE users (
    id VARCHAR(32) PRIMARY KEY NOT NULL DEFAULT (UUID_TO_BIN(UUID())),
    name TEXT NOT NULL,
    email VARCHAR(256) NOT NULL,
    provider ENUM('google') NOT NULL,
    provider_id VARCHAR(256) NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE groups (
    id VARCHAR(32) PRIMARY KEY NOT NULL DEFAULT (UUID_TO_BIN(UUID())),
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE user_groups (
   id VARCHAR(16) PRIMARY KEY NOT NULL DEFAULT (UUID_TO_BIN(UUID())),
   role ENUM('owner', 'member') NOT NULL DEFAULT 'member',
   user_id VARCHAR(32) NOT NULL REFERENCES users (id),
   group_id VARCHAR(32) NOT NULL REFERENCES groups (id),
   created_at DATETIME NOT NULL,

   UNIQUE (role, user_id, group_id)
);

CREATE INDEX users_email_idx ON users (email);
CREATE INDEX users_provider_idx ON users (provider, provider_id);

CREATE INDEX user_groups_user_id_idx ON user_groups (user_id);
CREATE INDEX user_groups_group_id_idx ON user_groups (group_id);
