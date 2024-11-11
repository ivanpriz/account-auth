use std::{ops::DerefMut, sync::Arc};

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use tokio::sync::RwLock;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::{
    application::{
        commands::{authenticate_user_command, create_user_command},
        dtos::user::{SignInData, UserCreateDTO, UserOutDTO},
        queries::get_user_query,
    },
    config::CONFIG,
    infrastructure::data::repositories::UsersRepository,
};
use framework::infrastructure::data::{
    db_context::connect_to_db,
    unit_of_work::{traits::UnitOfWorkFactoryInfraT, UnitOfWorkFactory},
};

#[debug_handler]
#[utoipa::path(post, path="/users", responses((status = OK, body = UserOutDTO)))]
pub async fn create_user_handler(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(user_create_data): Json<UserCreateDTO>,
) -> impl IntoResponse {
    let mut app_state_guard = app_state.write().await;
    let app_state = app_state_guard.deref_mut();
    let created_user = create_user_command(
        user_create_data,
        &mut app_state.users_repository,
        &mut app_state.uow_factory,
    )
    .await
    .expect("Failed to create user");
    Json(created_user)
}

#[debug_handler]
#[utoipa::path(get, path="/users/{id}", responses((status = OK, body = UserOutDTO), (status = 404, body = String), (status = 500, body = String)))]
pub async fn get_user_handler(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> axum::response::Response {
    let mut app_state_guard = app_state.write().await;
    let app_state = app_state_guard.deref_mut();
    let user = get_user_query(
        &id,
        &mut app_state.users_repository,
        &mut app_state.uow_factory,
    )
    .await;

    match user {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response(),
    }
}

#[debug_handler]
#[utoipa::path(post, path="/auth", responses((status = OK, body = String)))]
pub async fn authenticate_user_handler(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(sign_in_data): Json<SignInData>,
) -> impl IntoResponse {
    let mut app_state_guard = app_state.write().await;
    let app_state = app_state_guard.deref_mut();
    let auth_token = authenticate_user_command(
        sign_in_data,
        &mut app_state.users_repository,
        &mut app_state.uow_factory,
    )
    .await
    .expect("Failed to authenticate user");
    auth_token
}

pub struct AppState {
    pub users_repository: UsersRepository,
    pub uow_factory: UnitOfWorkFactory,
}

impl AppState {
    pub fn new(users_repository: UsersRepository, uow_factory: UnitOfWorkFactory) -> Self {
        Self {
            users_repository,
            uow_factory,
        }
    }
}

pub async fn create_router() -> Router {
    let pool = connect_to_db(&CONFIG.database_uri).await;
    let unit_of_work_factory = UnitOfWorkFactory::new(pool);
    let users_repo = UsersRepository {};
    let app_state = AppState::new(users_repo, unit_of_work_factory);
    let (router, api) = OpenApiRouter::new()
        .routes(routes!(create_user_handler, get_user_handler))
        .routes(routes!(authenticate_user_handler))
        .with_state(Arc::new(RwLock::new(app_state)))
        .split_for_parts();
    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));
    router
}
