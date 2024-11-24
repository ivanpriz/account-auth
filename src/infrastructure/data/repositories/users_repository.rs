use crate::application::specifications::UsersSpecification;
use crate::domain::models::user::User;
use framework::{
    application::{specifications::CompType, traits::RepositoryT},
    infrastructure::data::unit_of_work::{traits::UnitOfWorkInfraT, UnitOfWork},
};

pub struct UsersRepository();

impl<'tr> RepositoryT<User, UsersSpecification, UnitOfWork<'tr>> for UsersRepository {
    async fn create(&self, entity: &User, uow: &mut UnitOfWork<'tr>) -> User {
        let tx = uow.get_transaction();
        let user = match entity.id {
            None => sqlx::query_as!(
                User,
                r#"
                INSERT INTO users(username, hashed_password)
                VALUES ($1, $2)
                RETURNING id, username, hashed_password
            "#,
                entity.username,
                entity.hashed_password,
            )
            .fetch_one(&mut **tx)
            .await
            .expect("Couldn't create user"),
            Some(id) => sqlx::query_as!(
                User,
                r#"
                INSERT INTO users(id, username, hashed_password)
                VALUES ($1, $2, $3)
                RETURNING id, username, hashed_password
            "#,
                id,
                entity.username,
                entity.hashed_password,
            )
            .fetch_one(&mut **tx)
            .await
            .expect("Couldn't create user"),
        };
        user
    }

    async fn get_one_by(
        &self,
        specification: UsersSpecification,
        uow: &mut UnitOfWork<'tr>,
    ) -> Option<User> {
        let tx = uow.get_transaction();
        let user = match specification {
            UsersSpecification::Id(CompType::Equals(user_id)) => sqlx::query_as!(
                User,
                r#"
                    SELECT id, username, hashed_password
                    FROM users
                    WHERE users.id = $1
                "#,
                uuid::Uuid::from(user_id)
            )
            .fetch_optional(&mut **tx)
            .await
            .expect("Couldn't query user"),
            UsersSpecification::Username(CompType::Equals(username)) => sqlx::query_as!(
                User,
                r#"
                    SELECT id, username, hashed_password
                    FROM users
                    WHERE users.username = $1
                "#,
                username,
            )
            .fetch_optional(&mut **tx)
            .await
            .expect("Couldn't query user"),
            _ => panic!("Unsupported specification for querying one user!"),
        };
        user
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use framework::{
        application::{
            specifications::CompType,
            traits::{RepositoryT, UnitOfWorkFactoryT, UnitOfWorkT},
        },
        infrastructure::data::unit_of_work::UnitOfWorkFactory,
        test_utils::{migrations, uow_factory, WithCleanup},
    };
    use rstest::rstest;
    use serial_test::serial;

    use tokio::runtime::Runtime;
    use uuid::Uuid;

    use super::super::test_fixtures::existing_users;
    use crate::infrastructure::data::repositories::UsersRepository;
    use crate::{application::specifications::UsersSpecification, domain::models::user::User};

    #[rstest]
    #[serial(existing_users)]
    fn test_get_user_should_none(
        _migrations: WithCleanup<()>,
        uow_factory: (UnitOfWorkFactory, Runtime),
    ) {
        // println!("Entering test_get_user_should_none");
        let (mut uow_factory, runtime) = uow_factory;

        let mut uow = uow_factory.create_unit_of_work();
        let users_repo = UsersRepository {};
        runtime.block_on(uow.begin());
        let user = runtime.block_on(users_repo.get_one_by(
            UsersSpecification::Username(CompType::Equals(String::from("nonexistingmail"))),
            &mut uow,
        ));
        runtime.block_on(uow.rollback());
        assert_eq!(user, None);
        // println!("Finished test_get_user_should_none");
    }

    #[rstest]
    #[serial(existing_users)]
    fn test_get_user_should_some(
        _migrations: WithCleanup<()>,
        uow_factory: (UnitOfWorkFactory, Runtime),
        existing_users: WithCleanup<HashMap<Uuid, User>>,
        // default_users: HashMap<Uuid, User>,
    ) {
        // println!("Entering test_get_user_should_some");
        let (mut uow_factory, runtime) = uow_factory;
        for (_user_id, user) in existing_users._val.iter() {
            // Testing getting by email
            // println!("We have user in default users val: {:?}", user);
            let mut uow = uow_factory.create_unit_of_work();
            let users_repo = UsersRepository {};
            runtime.block_on(uow.begin());
            let user_from_repo = runtime.block_on(users_repo.get_one_by(
                UsersSpecification::Username(CompType::Equals(user.username.clone())),
                &mut uow,
            ));
            // println!("User we got from repo: {:?}", user_from_repo);
            runtime.block_on(uow.rollback());
            assert_eq!(Some(user), Some(&(user_from_repo.unwrap())));

            if let Some(uid) = user.id {
                // Testing getting by id
                let mut uow = uow_factory.create_unit_of_work();
                let users_repo = UsersRepository {};
                runtime.block_on(uow.begin());
                let user_from_repo = runtime.block_on(
                    users_repo.get_one_by(UsersSpecification::Id(CompType::Equals(uid)), &mut uow),
                );
                // println!("User we got from repo: {:?}", user_from_repo);
                runtime.block_on(uow.rollback());
                assert_eq!(Some(user), Some(&(user_from_repo.unwrap())));
            }
        }
        // println!("Finished test_get_user_should_some");
    }
}
