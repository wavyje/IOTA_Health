use actix_web::{get, web::{self, route, service}};
use actix_files as fs;
mod index;
mod home;
mod register;
mod authenticate;
use authenticate::{authenticate, process_authenticate};

/*mod content_loader;
mod profile;
mod scan;
mod success;
mod offline;*/
use super::path::Path;
use actix_web::{HttpRequest, Result};
use actix_files::{Files, NamedFile};

pub fn app_factory(app: &mut web::ServiceConfig) {
 let base_path: Path = Path{prefix: String::from("/")};

 app.service(
  Files::new("/static", "static")
  .show_files_listing(),
  )

 .route("/",
 web::get().to(index::index))

 .route(&base_path.define(String::from("register")),
  web::get().to(register::register))

  .route(&base_path.define(String::from("home")),
  web::get().to(home::home))

  .route(&base_path.define(String::from("authenticate")),
  web::get().to(authenticate::authenticate))

  .route(&base_path.define(String::from("process_authenticate")),
  web::post().to(authenticate::process_authenticate));
 /*.route(&base_path.define(String::from("profile")),
  web::get().to(profile::profile))

 .route(&base_path.define(String::from("scan")),
  web::get().to(scan::scan))

  .route(&base_path.define(String::from("success")),
  web::get().to(success::success));*/

}