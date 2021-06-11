use actix_web::{App, HttpServer};

mod views;
mod iota_logic;
use iota_logic::{initiate::initiate, client::create_client, check_channel::importauthor};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    //For more information on this visit README->chapter: 1) Getting started
    //WARNING: Only uncomment the following line, if you are absolutely sure!!!
    //let transport = create_client();
    //initiate(transport);
    //importauthor(transport);


    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app
    })
        //192.168.0.42
        .bind("192.168.0.42:8080")?
        .run()
        .await
}