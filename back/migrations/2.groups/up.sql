CREATE TABLE `grups` (
    id int PRIMARY KEY AUTO_INCREMENT,
    name TEXT NOT NULL,
    creator int NOT NULL

    FOREIGN KEY (creator) REFERENCES `users`(id)
);