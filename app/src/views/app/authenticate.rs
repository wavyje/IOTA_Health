//this class is for authenticating the user is an office member or a doctor
use actix_web::Result;
use actix_files::NamedFile;

pub async fn authenticate() ->  Result<NamedFile> {
    println!("authenticate");
    Ok(NamedFile::open("./static/authenticate.html")?)
}