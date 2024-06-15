-- Your SQL goes here
CREATE TABLE `users`(
	`id` INTEGER PRIMARY KEY,
	`first_name` TEXT NOT NULL,
	`last_name` TEXT NOT NULL,
	`creation_date` TIMESTAMP NOT NULL
);

CREATE TABLE `messages`(
	`id` INTEGER PRIMARY KEY,
	`content` TEXT NOT NULL,
	`author_id` INTEGER NOT NULL,
	`creation_date` TIMESTAMP NOT NULL,
	FOREIGN KEY (`author_id`) REFERENCES `users`(`id`)
);

