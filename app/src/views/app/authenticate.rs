//this class is for authenticating the user is an office member or a doctor
use actix_files::NamedFile;
use actix_web::{self, App, HttpResponse, HttpServer, Result, http, post, web::{self, Form}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    password: String,
}

pub async fn authenticate() ->  Result<NamedFile> {
   
    
    Ok(NamedFile::open("./static/authenticate.html")?)
}



//#[post("/process_authenticate")]
pub async fn process_authenticate(form: web::Form<FormData>) -> Result<HttpResponse>{
        println!("here");
        println!("{}", form.password);
        Ok(HttpResponse::Found().header(http::header::LOCATION, "/").finish())
}   