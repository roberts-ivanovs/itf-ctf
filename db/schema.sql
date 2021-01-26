
CREATE DATABASE IF NOT EXISTS `actix_todo`;
USE `actix_todo`;

DROP TABLE IF EXISTS `subtask`;
DROP TABLE IF EXISTS `todo`;
DROP DATABASE actix_todo;
CREATE DATABASE actix_todo;
USE actix_todo;

-- BASIC SCHEMA

CREATE TABLE users (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`email` VARCHAR(256) NOT NULL UNIQUE
);

CREATE TABLE flag (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL UNIQUE,
    `answer` VARCHAR(256) NOT NULL,
    `description` VARCHAR(32765) NULL,
    `filepath` VARCHAR(256) NOT NULL UNIQUE
);

CREATE TABLE score (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`flag_id` BIGINT UNSIGNED NOT NULL,
    `user_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`flag_id`) REFERENCES `users`(`id`),
    FOREIGN KEY (`user_id`) REFERENCES `flag`(`id`)
);


-- SAMPLE DATA
INSERT INTO flag(name, answer, filepath, description )
VALUES
    ("uzdevums 1", "correct", "task1.zip", "Apraksts nr 1"),
    ("uzdevums 3", "corrects", "task3.zip", "Apraksts nr 3 loti gars"),
    ("uzdevums 2", "asdf", "task2.zip", "Apraksts nr 2");

INSERT INTO users (email)
VALUES
    ("akmens@asd.lv"),
    ("ivans@asd.lv");

INSERT INTO score(flag_id, user_id)
VALUES
    (1, 1),
    (2, 1),
    (2, 2);
