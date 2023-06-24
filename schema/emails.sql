CREATE TABLE `emails` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `address` varchar(320) NOT NULL,
    `created` datetime NOT NULL,

    PRIMARY KEY (`id`),
    UNIQUE KEY `emails_idx_address` (`address`)
);
