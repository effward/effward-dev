CREATE TABLE commentContentHistories (
    id bigint NOT NULL AUTO_INCREMENT,
    commentId bigint NOT NULL,
    contentId bigint NOT NULL,
    created datetime NOT NULL,

    PRIMARY KEY(id)
);

CREATE INDEX commentContentHistories_idx_commentId on commentContentHistories (commentId);