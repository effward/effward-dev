CREATE TABLE emails (
    id bigint NOT NULL AUTO_INCREMENT,
    address varchar(320) NOT NULL,
    created datetime NOT NULL,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX emails_idx_address ON emails (address);
