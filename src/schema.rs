table! {
    subject (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        grade -> Nullable<Integer>,
        content -> Nullable<Text>,
    }
}

table! {
    teacher (id_teacher, id_subject) {
        id_teacher -> Unsigned<Integer>,
        id_subject -> Unsigned<Integer>,
    }
}

joinable!(teacher -> subject (id_subject));

allow_tables_to_appear_in_same_query!(
    subject,
    teacher,
);
