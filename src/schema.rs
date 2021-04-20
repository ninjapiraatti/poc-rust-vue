table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        password_plain -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    projects (pid) {
        pid -> Uuid,
        available -> Nullable<Bool>,
        name -> Varchar,
    }
}

table! {
    users (uid) {
        uid -> Uuid,
        isadmin -> Nullable<Bool>,
        ispro -> Nullable<Bool>,
        available -> Nullable<Bool>,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    invitations,
    projects,
    users,
);
