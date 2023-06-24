CREATE TABLE `comment_content_audit` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `comment_id` bigint NOT NULL,
    `content_id` bigint NOT NULL,
    `created` datetime NOT NULL,

    PRIMARY KEY (`id`),
    KEY `comment_content_audit_idx_comment_id` (`comment_id`)
);
