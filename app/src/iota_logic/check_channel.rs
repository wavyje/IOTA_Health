use std::{str::FromStr, vec};

use iota_streams::{app::{message::HasLink, transport::tangle::{PAYLOAD_BYTES, TangleAddress, TangleMessage, MsgId, AppInst}}, app_channels::{api::tangle::{
    Author,
    Subscriber,
}, message::announce}, core::{
prelude::Rc,
print,
println,
try_or,
LOCATION_LOG,
Errors::*,
}, ddml::types::*};

use iota_streams::{
app::transport::{
TransportOptions,
tangle::client::{SendOptions, Client, },
},
app_channels::api::tangle::Transport,
core::{
prelude::{ String},
Result,
},
};
use rand::AsByteSliceMut;
use core::cell::RefCell;

#[tokio::main]
pub async fn importauthor(transport: Rc<RefCell<Client>>, password: &str) -> bool{
    
    let mut by: Vec<u8> = vec![0, 0, 50, 34, 178, 106, 134, 237, 252, 34, 246, 246, 89, 119, 219, 115, 50, 18, 78, 206, 219, 12, 190, 15, 33, 2, 109, 39, 40, 198, 222, 85, 248, 81, 1, 1, 5, 117, 116, 102, 45, 56, 0, 0, 0, 0, 0, 0, 4, 66, 1, 225, 153, 155, 101, 26, 28, 179, 64, 173, 81, 191, 118, 196, 146, 149, 238, 78, 254, 85, 79, 178, 160, 36, 147, 125, 133, 51, 14, 21, 189, 184, 255, 
    0, 0, 0, 0, 0, 0, 0, 0, 191, 89, 99, 39, 166, 40, 133, 106, 107, 200, 126, 240, 1, 225, 153, 155, 101, 26, 28, 179, 64, 173, 81, 191, 118, 196, 146, 149, 238, 78, 254, 85, 79, 178, 160, 36, 147, 125, 133, 51, 14, 21, 189, 184, 255, 1, 1, 191, 89, 99, 39, 166, 40, 133, 106, 107, 200, 126, 240, 92, 200, 221, 167, 8, 93, 54, 55, 147, 63, 194, 79, 172, 254, 174, 218, 34, 9, 120, 220, 128, 199, 21, 217, 90, 37, 96, 205, 189, 41, 31, 221, 0, 0, 1, 1, 225, 153, 155, 101, 26, 28, 179, 64, 173, 81, 191, 118, 196, 146, 149, 238, 78, 254, 85, 79, 178, 160, 36, 147, 125, 133, 51, 14, 21, 189, 184, 255, 191, 89, 99, 39, 166, 40, 133, 106, 107, 200, 126, 240, 0, 0, 0, 0, 0, 0, 0, 2, 100, 253, 77, 9, 9, 213, 67, 35, 28, 161, 214, 23, 130, 213, 233, 96, 53, 187, 208, 23, 79, 147, 11, 89, 76, 61, 213, 26, 208, 199, 254, 116];
    let a = by.as_byte_slice_mut();

    let mut author = Author::import(a, password, transport.clone()).unwrap(); 
    println!("AppInst: {}", &author.channel_address().unwrap());
    //let appinst = author.;
    //author.send_keyload_for_everyone(TangleAddress::from_base_rel(&appinst,"7a1540ad28ad076dea7f6d25a6018941cba5a5744e189c7088b1a3780f24476d"));


    let app = AppInst::from_str("e1999b651a1cb340ad51bf76c49295ee4efe554fb2a024937d85330e15bdb8ff0000000000000000").unwrap();
    let msg = MsgId::from_str("bf596327a628856a6bc87ef0").unwrap();
    let announce_link = TangleAddress::from_base_rel(&app, &msg);
    println!("Address: {}", &announce_link);

    return true;
    
    /*let keyload_link = {
        let (msg, seq)= author.send_keyload_for_everyone(&announce_link).unwrap();
        let seq = seq.unwrap();
        println!("  msg => <{}> {:?}", msg.msgid, msg);
        println!("  seq => <{}> {:?}", seq.msgid, seq);
        println!();
        seq
        };
    println!("Keyload link: {}", &keyload_link);*/
    //println!("AppInst: {} ; Msgid: {}", base, msgid);
}