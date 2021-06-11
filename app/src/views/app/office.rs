use actix_web::Result;
use actix_files::NamedFile;

pub async fn office() ->  Result<NamedFile> {
    println!("office");
    Ok(NamedFile::open("./static/office.html")?)
}