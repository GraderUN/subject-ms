-- Your SQL goes here
CREATE TABLE subject (
    id INT unsigned NOT NULL AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL DEFAULT '',
    grade INT DEFAULT NULL,
    content TEXT DEFAULT '',
    PRIMARY KEY (id)
);
