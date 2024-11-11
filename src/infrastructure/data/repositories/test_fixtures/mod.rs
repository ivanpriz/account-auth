use std::collections::HashMap;

// use chrono::NaiveDate;
use rstest::fixture;
use sqlx::PgPool;
use tokio::runtime::Runtime;
use uuid::Uuid;

use framework::test_utils::{pg_pool, WithCleanup};

use crate::domain::models::user::User;

#[fixture]
pub fn default_users() -> HashMap<Uuid, User> {
    let id1 = Uuid::new_v4();
    HashMap::from([(
        id1,
        User {
            id: Some(id1),
            email: String::from("mail@mail.com"),
            hashed_password: String::from("hashed_pwd"),
        },
    )])
}

#[fixture]
pub fn existing_users(
    pg_pool: (PgPool, Runtime),
    default_users: HashMap<Uuid, User>,
) -> WithCleanup<HashMap<Uuid, User>> {
    println!("Creating default users...");
    let (pool, runtime) = pg_pool;

    for (_id, user) in default_users.iter() {
        runtime
            .block_on(
                sqlx::query!(
                    r#"
                    INSERT INTO users(id, email, hashed_password)
                    VALUES ($1, $2, $3)
                    RETURNING id, email, hashed_password
                "#,
                    user.id,
                    user.email,
                    user.hashed_password,
                )
                .fetch_one(&pool),
            )
            .expect("Couldn't create user");
    }

    let ids = default_users
        .keys()
        .map(|key| key.clone())
        .collect::<Vec<Uuid>>();

    println!("Default users created, ids: {:?}", ids);

    WithCleanup {
        _val: default_users,
        closure: Box::new(move || {
            println!("Deleting default users with ids: {:?}", ids);
            runtime
                .block_on(
                    sqlx::query!(
                        r#"
                            DELETE FROM users
                            WHERE id = ANY($1)
                            "#,
                        &ids[..]
                    )
                    .execute(&pool),
                )
                .expect("Error while deleting default users");
            println!("Deleted default users with ids: {:?}", ids);
        }),
    }
}
