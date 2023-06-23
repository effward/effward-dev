CREATE TABLE contents (
    id bigint NOT NULL AUTO_INCREMENT,
    body mediumtext NOT NULL,
    bodyHash char(32) NOT NULL,
    created datetime NOT NULL,
    
    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX contents_idx_bodyHash ON contents (bodyHash);