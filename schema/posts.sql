CREATE TABLE posts (
    id bigint NOT NULL AUTO_INCREMENT,
    publicId binary(16) NOT NULL,
    author bigint NOT NULL,
    title varchar(512) NOT NULL,
    link varchar(1024) NULL,
    contentId bigint NULL,
    created datetime NOT NULL,
    updated datetime NOT NULL,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX posts_idx_publicId ON posts (publicId);
CREATE INDEX posts_idx_created ON posts (created);
CREATE INDEX posts_idx_title ON posts (title);
CREATE INDEX posts_idx_author ON posts (author);