-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` VARCHAR(64) NOT NULL UNIQUE,
	`email` VARCHAR(256) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS todo (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` VARCHAR(64) NOT NULL UNIQUE,
    `owener_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`owener_id`) REFERENCES `users`(`id`)
);

CREATE TABLE IF NOT EXISTS subtask (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` VARCHAR(64) NOT NULL UNIQUE,
    `owener_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`owener_id`) REFERENCES `users`(`id`)
);
