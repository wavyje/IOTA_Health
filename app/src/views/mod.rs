mod app;
mod path;
use actix_web::web;

use self::app::app_factory;

pub fn views_factory(app: &mut web::ServiceConfig) {
    //auth_factory
    app_factory(app);
}