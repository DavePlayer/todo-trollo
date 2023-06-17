CREATE TABLE `group_assigned_users` (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    group_id int NOT NULL,
    user_id int NOT NULL,
    FOREIGN KEY (group_id) REFERENCES `grups`(id),
    FOREIGN KEY (user_id) REFERENCES `users`(id)
);