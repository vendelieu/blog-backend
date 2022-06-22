use crate::{
    configurations::db::Pool,
    consts,
    utils::error_handling::ServiceError,
    models::{
        filters::PostFilter,
        post::{Post, PostDTO},
        response::Page,
    },
    services::user_service,
};
use actix_identity::Identity;
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Post>, ServiceError> {
    match Post::find_all(&pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_slug(slug: String, pool: &web::Data<Pool>) -> Result<Post, ServiceError> {
    match Post::find_by_slug(&slug, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Post with slug {} not found", &slug),
        )),
    }
}

pub fn filter(filter: PostFilter, pool: &web::Data<Pool>) -> Result<Page<Post>, ServiceError> {
    match Post::filter(filter, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new_post: PostDTO, id: Identity, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Err(err) = user_service::check_is_admin(
        id, &pool.get().unwrap(),
    ) { return Err(err); }

    match Post::insert(new_post, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(
    id: i32,
    updated_post: PostDTO,
    identity: Identity,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    if let Err(err) = user_service::check_is_admin(
        identity, &pool.get().unwrap(),
    ) { return Err(err); }

    match Post::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Post::update(id, updated_post, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Post with id {} not found", id),
        )),
    }
}

pub fn delete(id: i32, identity: Identity, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Err(err) = user_service::check_is_admin(
        identity, &pool.get().unwrap(),
    ) { return Err(err); }

    match Post::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Post::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Post with id {} not found", id),
        )),
    }
}