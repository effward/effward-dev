CREATE TABLE comments (
    id bigint NOT NULL AUTO_INCREMENT,
    publicId binary(16) NOT NULL,
    author bigint NOT NULL,
    postId bigint NOT NULL,
    parentId bigint NULL,
    contentId bigint NOT NULL,
    created datetime NOT NULL,
    updated datetime NOT NULL,
    
    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX comments_idx_publicId ON comments (publicId);
CREATE INDEX comments_idx_postId ON comments (postId);
CREATE INDEX comments_idx_author ON comments (author);