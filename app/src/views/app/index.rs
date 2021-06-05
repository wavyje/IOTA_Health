use actix_web::Result;
use actix_files::NamedFile;

pub async fn index() ->  Result<NamedFile> {
    println!("index");
    Ok(NamedFile::open("./static/index.html")?)
}