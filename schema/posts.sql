CREATE TABLE `posts` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `public_id` binary(16) NOT NULL,
    `author` bigint unsigned NOT NULL,
    `title` varchar(512) NOT NULL,
    `link` varchar(1024) NULL,
    `content_id` bigint unsigned NULL,
    `created` datetime NOT NULL,
    `updated` datetime NOT NULL,

    PRIMARY KEY (`id`),
    UNIQUE KEY `posts_idx_public_id` (`public_id`),
    KEY `posts_idx_created` (`created`),
    KEY `posts_idx_title` (`title`),
    KEY `posts_idx_author` (`author`)
);
