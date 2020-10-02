-- Your SQL goes here
CREATE TABLE teacher (
    id_teacher INT unsigned NOT NULL,
    id_subject INT unsigned NOT NULL,
    PRIMARY KEY (id_teacher, id_subject),
    FOREIGN KEY (id_subject) REFERENCES subject(id)
);
