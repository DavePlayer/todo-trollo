CREATE TABLE `tasks` (
    id int PRIMARY KEY AUTO_INCREMENT,
    name TEXT NOT NULL,
    crossed_by_id int,
    group_id int NOT NULL,

    FOREIGN KEY (group_id) REFERENCES `grups`(id),
    FOREIGN KEY (crossed_by_id) REFERENCES `users`(id)
);