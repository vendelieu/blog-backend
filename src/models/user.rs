use crate::{
    configurations::db::Connection,
    consts,
    schema::users::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: Option<String>,
    pub is_admin: bool,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &Connection) -> Result<String, String> {
        if Self::find_user_by_username(&user.username, conn).is_err() {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UserDTO {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users).values(&user).execute(conn);
            Ok(consts::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("User '{}' is already registered", &user.username))
        }
    }

    pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.is_empty()
                && verify(&login.password, &user_to_verify.password).unwrap()
            {
                let login_session_str = User::generate_login_session();
                if User::update_login_session_to_db(
                    &user_to_verify.username,
                    &login_session_str,
                    conn,
                ) {
                    return Some(
                        LoginInfoDTO {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        }
                    );
                }
            } else {
                return Some(
                    LoginInfoDTO {
                        username: user_to_verify.username,
                        login_session: String::new(),
                    }
                );
            }
        }

        None
    }

    pub fn logout(user_id: i32, conn: &Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.username, "", conn);
        }
    }

    pub fn is_login_session_valid(uname: &str, lgn_session: &str, conn: &Connection) -> bool {
        users
            .filter(username.eq(&uname))
            .filter(login_session.eq(&lgn_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().as_simple().to_string()
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}
