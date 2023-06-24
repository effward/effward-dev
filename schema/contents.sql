CREATE TABLE `contents` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `body` mediumtext NOT NULL,
    `body_hash` char(32) NOT NULL,
    `created` datetime NOT NULL,
    
    PRIMARY KEY (`id`),
    UNIQUE KEY `contents_idx_body_hash` (`body_hash`)
);
