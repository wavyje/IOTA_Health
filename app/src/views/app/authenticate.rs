//this class is for authenticating the user is an office member or a doctor
use actix_files::NamedFile;
use actix_web::{self, App, HttpResponse, HttpServer, Result, http, post, web::{self, Form}};
use serde::Deserialize;

use crate::iota_logic::client;
use crate::iota_logic::check_channel::importauthor;


#[derive(Deserialize)]
pub struct FormData {
    password: String,
}

pub async fn authenticate() ->  Result<NamedFile> {
    Ok(NamedFile::open("./static/authenticate.html")?)
}




pub async fn process_authenticate(form: web::Form<FormData>) -> Result<HttpResponse>{
        //build IOTA Client
        let transport = client::create_client();
        //try importing user
        if(importauthor(transport, &form.password)) {
            println!("{}", form.password);
            Ok(HttpResponse::Found().header(http::header::LOCATION, "/office").finish())
        }
        else {
            let err = "Login fehlgeschlagen!";
            Ok(HttpResponse::Ok().body(err))
        }
        
}   