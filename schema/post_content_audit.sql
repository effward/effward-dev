CREATE TABLE `post_content_audit` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `post_id` bigint NOT NULL,
    `content_id` bigint NOT NULL,
    `created` datetime NOT NULL,
    
    PRIMARY KEY (`id`),
    KEY `post_content_audit_idx_post_id` (`post_id`)
);
