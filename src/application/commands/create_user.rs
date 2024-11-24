use crate::application::dtos::user::{UserCreateDTO, UserOutDTO};
use crate::application::specifications::UsersSpecification;
use crate::domain::models::user::User;
use crate::utils::hash_pwd;
use framework::application::traits::{RepositoryT, UnitOfWorkFactoryT, UnitOfWorkT};

pub async fn create_user_command<UnitOfWorkType: UnitOfWorkT>(
    user_create_data: UserCreateDTO,
    users_repository: &mut impl RepositoryT<User, UsersSpecification, UnitOfWorkType>,
    uow_factory: &mut impl UnitOfWorkFactoryT<UnitOfWorkType>,
) -> Result<UserOutDTO, String> {
    let mut uow = uow_factory.create_unit_of_work();
    uow.begin().await;

    let user = users_repository
        .create(
            &User {
                id: None,
                username: user_create_data.username,
                hashed_password: hash_pwd(&user_create_data.password),
            },
            &mut uow,
        )
        .await;

    uow.commit().await;
    Ok(UserOutDTO {
        id: user.id,
        username: user.username,
    })
}
