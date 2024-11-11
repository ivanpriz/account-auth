// TODO think if we want it as a struct or as function.
// If we have it as a struct we can try storing handlers in app state.
// However this might be complex in terms of borrow checker fighting.
// So, probably just having functions we will call in handlers providing repos and uow factory from app state inside handler.
// pub struct CreateUserCommandHandler<
//     'u_r,
//     'uow_f,
//     UnitOfWorkType: IUnitOfWork,
//     UsersRepoType: IRepository<User, UsersSpecification, UnitOfWorkType>,
//     UnitOfWorkFactoryType: IUnitOfWorkFactory<UnitOfWorkType>,
// > {
//     pub users_repository: &'u_r UsersRepoType,
//     pub uow_factory: &'uow_f mut UnitOfWorkFactoryType,
//     _marker: marker::PhantomData<UnitOfWorkType>,
// }

// impl<
//         'u_r,
//         'uow_f,
//         UnitOfWorkType: IUnitOfWork,
//         UsersRepoType: IRepository<User, UsersSpecification, UnitOfWorkType>,
//         UnitOfWorkFactoryType: IUnitOfWorkFactory<UnitOfWorkType>,
//     > CreateUserCommandHandler<'u_r, 'uow_f, UnitOfWorkType, UsersRepoType, UnitOfWorkFactoryType>
// {
//     pub fn new(
//         users_repository: &'u_r UsersRepoType,
//         uow_factory: &'uow_f mut UnitOfWorkFactoryType,
//     ) -> Self {
//         Self {
//             users_repository,
//             uow_factory,
//             _marker: PhantomData,
//         }
//     }
//     pub async fn handle(&mut self, user_create_data: UserCreateDTO) -> Result<UserOutDTO, String> {
//         let mut uow = self.uow_factory.create_unit_of_work();
//         uow.begin().await;

//         let user = self
//             .users_repository
//             .create(
//                 &User {
//                     id: None,
//                     email: user_create_data.email,
//                     hashed_password: String::from("tobe implemented"),
//                     first_name: user_create_data.first_name,
//                     last_name: user_create_data.last_name,
//                     birth_date: user_create_data.birth_date,
//                     gender_is_male: user_create_data.gender_is_male,
//                     interests: user_create_data.interests,
//                     city: user_create_data.city,
//                 },
//                 &mut uow,
//             )
//             .await;

//         uow.commit().await;
//         Ok(UserOutDTO {
//             id: user.id,
//             email: user.email,
//             first_name: user.first_name,
//             last_name: user.last_name,
//             birth_date: user.birth_date,
//             gender_is_male: user.gender_is_male,
//             interests: user.interests,
//             city: user.city,
//         })
//     }
// }
