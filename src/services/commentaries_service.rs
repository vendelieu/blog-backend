use crate::{
    configurations::db::Pool,
    consts,
    utils::error_handling::ServiceError,
    models::{
        commentaries::{Commentary, CommentaryFullDTO, CommentaryDTO},
    },
    services::user_service,
};
use actix_identity::Identity;
use actix_web::{http::StatusCode, web};
use crate::models::filters::CommentaryFilter;
use crate::models::post::Post;
use crate::models::response::Page;

pub fn filter_by_post_slug(
    slug: String, filter: CommentaryFilter, pool: &web::Data<Pool>,
) -> Result<Page<Commentary>, ServiceError> {
    match Commentary::filter_by_post_slug(&slug, filter, &pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(
    p_slug: String, new_comment: CommentaryDTO, id: Identity, pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    let user = match user_service::handle_user_auth(id, &pool.get().unwrap()) {
        Ok(user) => user,
        Err(err) => return Err(err)
    };

    // check if post exists & open for comments
    match Post::find_by_slug(&p_slug, &pool.get().unwrap()) {
        Ok(post) => if !post.commentaries_open {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
            ));
        }
        Err(_) => return Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        ))
    };

    match Commentary::insert(CommentaryFullDTO {
        post_slug: p_slug,
        username: user.username,
        text: new_comment.text,
        reply_to: new_comment.reply_to,
    }, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            consts::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(
    comm_id: i32,
    updated_comment: CommentaryDTO,
    identity: Identity,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    let user = match user_service::handle_user_auth(identity, &pool.get().unwrap()) {
        Ok(user) => user,
        Err(err) => return Err(err)
    };

    match Commentary::find_by_id(comm_id, &pool.get().unwrap()) {
        Ok(comm) => {
            // check is user trying to update others comment
            if comm.username != user.username {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                ));
            }
            // case when trying to reply to empty commentary
            if !match updated_comment.reply_to {
                Some(id) => Commentary::find_by_id(id, &pool.get().unwrap()).is_ok(),
                None => true
            } {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                ));
            }

            match Commentary::update(comm_id, CommentaryFullDTO {
                post_slug: comm.post_slug,
                username: comm.username,
                text: updated_comment.text,
                reply_to: updated_comment.reply_to,
            }, &pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                )),
            }
        }
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Comment with id {} not found", comm_id),
        )),
    }
}

pub fn delete(id: i32, identity: Identity, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    let user = match user_service::handle_user_auth(identity, &pool.get().unwrap()) {
        Ok(user) => user,
        Err(err) => return Err(err)
    };

    match Commentary::find_by_id(id, &pool.get().unwrap()) {
        Ok(comm) => {
            // check is user trying to delete others comment
            if comm.username != user.username {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
                ));
            }

            match Commentary::delete(id, &pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    consts::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
                )),
            }
        }
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Comment with id {} not found", id),
        )),
    }
}