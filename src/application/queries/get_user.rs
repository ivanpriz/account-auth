use framework::application::specifications::CompType;
use uuid::Uuid;

use framework::application::traits::{RepositoryT, UnitOfWorkFactoryT, UnitOfWorkT};

use crate::application::dtos::user::UserOutDTO;
use crate::application::specifications::UsersSpecification;
use crate::domain::models::user::User;

pub async fn get_user_query<UnitOfWorkType: UnitOfWorkT>(
    user_id: &Uuid,
    users_repository: &mut impl RepositoryT<User, UsersSpecification, UnitOfWorkType>,
    uow_factory: &mut impl UnitOfWorkFactoryT<UnitOfWorkType>,
) -> Result<Option<UserOutDTO>, String> {
    let mut uow = uow_factory.create_unit_of_work();
    uow.begin().await; // todo: non-transactional uows. Can wrap them into enum UOW{Transactional, NonTransactional}
    let user = users_repository
        .get_one_by(
            UsersSpecification::Id(CompType::Equals(user_id.clone())),
            &mut uow,
        )
        .await;
    uow.commit().await;
    // todo: maybe return domain entities from commands/queries and on presentation layer transform them to dtos.
    match user {
        Some(user) => Ok(Some(UserOutDTO {
            id: user.id,
            email: user.email,
        })),
        None => Ok(None),
    }
}
