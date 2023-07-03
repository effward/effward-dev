CREATE TABLE `comments` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `public_id` binary(16) NOT NULL,
    `author_id` bigint unsigned NOT NULL,
    `post_id` bigint unsigned NOT NULL,
    `parent_id` bigint unsigned NULL,
    `content_id` bigint unsigned NOT NULL,
    `created` datetime NOT NULL,
    `updated` datetime NOT NULL,
    
    PRIMARY KEY (`id`),
    UNIQUE KEY `comments_idx_public_id` (`public_id`),
    KEY `comments_idx_post_id` (`post_id`),
    KEY `comments_idx_author_id` (`author_id`),
    KEY `comments_idx_post_id_parent_id` (`post_id`, `parent_id`)
);
