CREATE TABLE users (
    id bigint NOT NULL AUTO_INCREMENT,
    publicId binary(16) NOT NULL,
    name varchar(64) NOT NULL,
    emailId bigint NOT NULL,
    password varchar(1024) NOT NULL, -- salt:hash:hashFunc
    isDeleted boolean,
    created datetime NOT NULL,
    updated datetime NOT NULL,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX users_idx_publicId ON users (publicId);
CREATE UNIQUE INDEX users_idx_name ON users (name);
CREATE INDEX users_idx_emailId ON users (emailId);
