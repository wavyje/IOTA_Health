use std::vec;

use iota_streams::{app::{message::HasLink, transport::tangle::{PAYLOAD_BYTES, TangleAddress, TangleMessage}}, app_channels::{api::tangle::{
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

pub fn importauthor(transport: Rc<RefCell<Client>>) {
    let mut by: Vec<u8> = vec![0, 0, 58, 47, 120, 58, 119, 0, 79, 216, 180, 153, 240, 29, 0, 29, 81, 180, 45, 74, 233, 147, 206, 71, 249, 164, 160, 38, 241, 184, 42, 139, 248, 129, 1, 1, 5, 117, 116, 102, 45, 56, 0, 0, 0, 0, 0, 0, 4, 66, 1, 235, 155, 80, 152, 94, 166, 21, 204, 213, 56, 210, 19, 246, 132, 161, 63, 179, 223, 82, 124, 132, 239, 204, 84, 232, 35, 203, 94, 126, 164, 80, 125, 
    0, 0, 0, 0, 0, 0, 0, 0, 82, 59, 121, 27, 25, 229, 255, 34, 176, 98, 69, 107, 1, 235, 155, 80, 152, 94, 166, 21, 204, 213, 56, 210, 19, 246, 132, 161, 63, 179, 223, 82, 124, 132, 239, 204, 84, 232, 35, 203, 94, 126, 164, 80, 125, 1, 1, 82, 59, 121, 27, 25, 229, 255, 34, 176, 98, 69, 107, 103, 241, 227, 173, 208, 7, 160, 192, 231, 47, 21, 155, 12, 221, 147, 213, 221, 177, 48, 229, 165, 200, 49, 74, 244, 140, 180, 164, 89, 95, 228, 220, 0, 0, 1, 1, 235, 155, 80, 152, 94, 166, 21, 204, 213, 56, 210, 19, 246, 132, 161, 63, 179, 223, 82, 124, 132, 239, 204, 84, 232, 35, 203, 94, 126, 164, 80, 125, 82, 59, 121, 27, 25, 229, 255, 34, 176, 98, 69, 107, 0, 0, 0, 0, 0, 0, 0, 2, 198, 221, 31, 159, 199, 206, 87, 58, 123, 180, 209, 0, 185, 200, 135, 19, 208, 161, 103, 198, 78, 145, 114, 163, 12, 222, 34, 115, 62, 126, 
    115, 0];
    let a = by.as_byte_slice_mut();

    let author = Author::import(a, "Geheimes Passwort", transport.clone()).unwrap();
    println!("Adddesed: {}", &author.channel_address().unwrap());
}