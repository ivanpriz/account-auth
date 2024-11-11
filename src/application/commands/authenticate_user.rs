use crate::application::specifications::UsersSpecification;
use crate::domain::models::user::User;
use crate::utils::encode_jwt;
use crate::{application::dtos::user::SignInData, utils::verify_pwd};
use framework::application::{
    specifications::CompType,
    traits::{RepositoryT, UnitOfWorkFactoryT, UnitOfWorkT},
};

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    PasswordIncorrect,
}

pub async fn authenticate_user_command<UnitOfWorkType: UnitOfWorkT>(
    sign_in_data: SignInData,
    users_repository: &mut impl RepositoryT<User, UsersSpecification, UnitOfWorkType>,
    uow_factory: &mut impl UnitOfWorkFactoryT<UnitOfWorkType>,
) -> Result<String, AuthError> {
    // Takes user email and password and returns token encoding the user id
    let mut uow = uow_factory.create_unit_of_work();
    uow.begin().await;

    let user = users_repository
        .get_one_by(
            UsersSpecification::Email(CompType::Equals(sign_in_data.email.clone())),
            &mut uow,
        )
        .await;

    let res = match user {
        None => Err(AuthError::UserNotFound),
        Some(user) => {
            if !verify_pwd(&sign_in_data.password, &user.hashed_password) {
                return Err(AuthError::PasswordIncorrect);
            }

            Ok(encode_jwt(&user.email).unwrap())
        }
    };

    uow.commit().await; // todo: need non-transactional uow support

    res
}
