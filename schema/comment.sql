CREATE TABLE `comments` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `public_id` binary(16) NOT NULL,
    `author` bigint NOT NULL,
    `post_id` bigint NOT NULL,
    `parent_id` bigint NULL,
    `content_id` bigint NOT NULL,
    `created` datetime NOT NULL,
    `updated` datetime NOT NULL,
    
    PRIMARY KEY (`id`),
    UNIQUE KEY `comments_idx_public_id` (`public_id`),
    KEY `comments_idx_post_id` (`post_id`),
    KEY `comments_idx_author` (`author`)
);
