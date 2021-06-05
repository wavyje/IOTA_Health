use actix_web::Result;
use actix_files::NamedFile;

pub async fn home() ->  Result<NamedFile> {
    println!("home");
    Ok(NamedFile::open("./static/home.html")?)
}