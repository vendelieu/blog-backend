use crate::{
    configurations::db::Pool,
    consts,
    utils::error_handling::ServiceError,
    models::user::{LoginDTO, User, UserDTO},
};
use actix_web::{
    http::{StatusCode},
    web,
};
use actix_identity::Identity;
use crate::configurations::db::Connection;
use crate::models::user::LoginInfoDTO;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
    }
}

pub fn login(login: LoginDTO, id: Identity, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::login(login, &pool.get().unwrap()) {
        Some(logged_user) => {
            id.remember(serde_json::to_string(&logged_user).unwrap());
            Ok(consts::MESSAGE_OK.to_string())
        }
        None => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            consts::MESSAGE_USER_NOT_FOUND.to_string(),
        ))
    }
}

pub fn logout(id: Identity, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    let user: LoginInfoDTO = serde_json::from_str(&id.identity().unwrap()).unwrap();
    if User::is_login_session_valid(
        &user.username, &user.login_session, &pool.get().unwrap(),
    ) {
        if let Ok(user) = User::find_user_by_username(
            &user.username, &pool.get().unwrap(),
        ) {
            User::logout(user.id, &pool.get().unwrap());
            id.forget();
            return Ok(());
        }
    }

    Err(ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        consts::MESSAGE_AUTH_PROCESS_ERROR.to_string(),
    ))
}

pub fn get_user_by_identity(id: Identity, conn: &Connection) -> Result<User, ServiceError> {
    let user: LoginInfoDTO = serde_json::from_str(
        match &id.identity() {
            Some(i) => i,
            None => return Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                consts::MESSAGE_USER_NOT_FOUND.to_string(),
            ))
        }
    ).unwrap();

    if !User::is_login_session_valid(
        &user.username, &user.login_session, conn,
    ) {
        return Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            consts::MESSAGE_USER_NOT_FOUND.to_string(),
        ));
    }

    match User::find_user_by_username(&user.username, conn) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            consts::MESSAGE_USER_NOT_FOUND.to_string(),
        ))
    }
}

pub fn handle_user_auth(id: Identity, conn: &Connection) -> Result<User, ServiceError> {
    match get_user_by_identity(id, conn) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::FORBIDDEN,
            consts::MESSAGE_USER_NOT_FOUND.to_string(),
        ))
    }
}

pub fn check_is_admin(id: Identity, conn: &Connection) -> Result<(), ServiceError> {
    match get_user_by_identity(id, conn) {
        Ok(user) => if !user.is_admin {
            Err(ServiceError::new(
                StatusCode::FORBIDDEN,
                consts::MESSAGE_NOT_ALLOWED.to_string(),
            ))
        } else { Ok(()) },
        Err(err) => Err(err)
    }
}