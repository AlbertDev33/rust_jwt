use actix_web::web;

use super::{
    helth_checker::health_checker_handler, register_user::register_user_handler,
    user_login::user_login_handler, user_logout::logout_handler, user_me::get_me_handler,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(user_login_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
