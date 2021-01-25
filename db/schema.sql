
CREATE DATABASE IF NOT EXISTS `actix_todo`;
USE `actix_todo`;

DROP TABLE IF EXISTS `subtask`;
DROP TABLE IF EXISTS `todo`;
DROP TABLE IF EXISTS `users`;


CREATE TABLE IF NOT EXISTS users (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`email` VARCHAR(256) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS flag (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL UNIQUE,
    `answer` VARCHAR(256) NOT NULL,
    `filepath` VARCHAR(256) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS flag_user_mapping (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`flag_id` BIGINT UNSIGNED NOT NULL,
    `user_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`flag_id`) REFERENCES `users`(`id`),
    FOREIGN KEY (`user_id`) REFERENCES `flag`(`id`)
);
