CREATE TABLE `posts` (
                         `id` INT(11) NOT NULL AUTO_INCREMENT,
                         `author_id` INT(11) NOT NULL,
                         `content` TEXT NULL DEFAULT NULL COLLATE 'utf8_general_ci',
                         PRIMARY KEY (`id`) USING BTREE,
                         INDEX `author_id` (`author_id`) USING BTREE,
                         CONSTRAINT `posts_ibfk_1` FOREIGN KEY (`author_id`) REFERENCES `users` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
)