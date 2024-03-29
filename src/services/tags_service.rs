use actix_web::{http::StatusCode, web};

use crate::{
    configurations::db::Pool,
    consts,
    models::{
        tags::{Tag, TagDTO},
    },
    utils::error_handling::ServiceError,
};
use crate::models::tags::PostTagsPivot;

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Tag>, ServiceError> {
    match Tag::find_all(&pool.get().unwrap()) {
        Ok(tags) => Ok(tags),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_name(name: String, pool: &web::Data<Pool>) -> Result<Vec<Tag>, ServiceError> {
    match Tag::find_by_name(&name, &pool.get().unwrap()) {
        Ok(tags) => Ok(tags),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Tag with name {} not found", &name),
        )),
    }
}

pub fn find_by_post_slug(slug: String, pool: &web::Data<Pool>) -> Result<Vec<Tag>, ServiceError> {
    match Tag::find_by_post_slug(&slug, &pool.get().unwrap()) {
        Ok(tags) => Ok(tags),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new_tag: TagDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Tag::insert(new_tag, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn link(new_pivot: PostTagsPivot, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Tag::link_with_post(new_pivot, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn unlink(t_slug: String, p_slug: String, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Tag::unlink_pivot(t_slug, p_slug, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(
    id: i32,
    updated_tag: TagDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Tag::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Tag::update(id, updated_tag, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Tag with id {} not found", id),
        )),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Tag::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Tag::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Tag with id {} not found", id),
        )),
    }
}