CREATE TABLE `users` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `public_id` binary(16) NOT NULL,
    `name` varchar(64) NOT NULL,
    `email_id` bigint NOT NULL,
    `password` varchar(1024) NOT NULL, -- hash:salt:hash_func
    `is_deleted` boolean NOT NULL,
    `created` datetime NOT NULL,
    `updated` datetime NOT NULL,

    PRIMARY KEY (`id`),
    UNIQUE KEY `users_idx_public_id` (`public_id`),
    UNIQUE KEY `users_idx_name` (`name`),
    KEY `users_idx_email_id` (`email_id`)
);
