use actix_web::Result;
use actix_files::NamedFile;

pub async fn register() ->  Result<NamedFile> {
    println!("register");
    Ok(NamedFile::open("./static/datainput.html")?)
}