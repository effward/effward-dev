CREATE TABLE postContentHistories (
    id bigint NOT NULL AUTO_INCREMENT,
    postId bigint NOT NULL,
    contentId bigint NOT NULL,
    created datetime NOT NULL,
    
    PRIMARY KEY(id)
);

CREATE INDEX postContentHistories_idx_postId on postContentHistories (postId);