use crate::{
    configurations::db::Pool,
    consts,
    utils::error_handling::ServiceError,
    models::{
        filters::PostFilter,
        post::{Post, PostDTO},
        response::Page,
    },
};
use actix_web::{http::StatusCode, web};
use crate::utils::db_nav_post_type_wrapper::NavPost;

pub fn find_by_slug(slug: String, pool: &web::Data<Pool>) -> Result<Post, ServiceError> {
    match Post::find_by_slug(&slug, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(err) => {
            eprintln!("Error at fetching post data by slug process: {}", err);
            Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                format!("Post with slug {} not found", &slug),
            ))
        }
    }
}

pub fn filter(filter: PostFilter, pool: &web::Data<Pool>) -> Result<Page<Post>, ServiceError> {
    match Post::filter(filter, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(err) => {
            eprintln!("Error at fetching post data process: {}", err);
            Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
            ))
        }
    }
}

pub fn get_related(slug: String, pool: &web::Data<Pool>) -> Result<Vec<NavPost>, ServiceError> {
    match Post::filter_related(slug, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(err) => {
            eprintln!("Error at fetching related post data process: {}", err);
            Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
            ))
        }
    }
}

pub fn insert(new_post: PostDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Post::insert(new_post, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error at inserting post data process: {}", err);
            Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
            ))
        }
    }
}

pub fn update(
    id: i32,
    updated_post: PostDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Post::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Post::update(id, updated_post, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error at updating post data process: {}", err);

                Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                ))
            }
        },
        Err(err) => {
            eprintln!("Error at updating post data process, post not found: {}", err);

            Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                format!("Post with id {} not found", id),
            ))
        }
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Post::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Post::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error at deleting post data process: {}", err);

                Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
                ))
            }
        },
        Err(err) => {
            eprintln!("Error at deleting post data process, post not found: {}", err);
            Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                format!("Post with id {} not found", id),
            ))
        }
    }
}