mod state_model;
mod osc_client;

use std::cell::RefCell;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::Arc;
use jdw_osc_lib::osc_stack::OSCStack;
use crate::osc_client::OscClient;
use crate::state_model::{Letter, LetterPayloads};

fn main() {

    let socket = UdpSocket::bind(
        SocketAddrV4::from_str("127.0.0.1:13459").unwrap()
    ).unwrap();

    let mut client = OscClient::new(
        socket,
        SocketAddrV4::from_str("127.0.0.1:13339").unwrap() // Router addr
    );

    let handle = RefCell::new(client);

    let letter_states = LetterPayloads::new();
    let states_handle = RefCell::new(letter_states);

    OSCStack::init("127.0.0.1:13458".to_string())
        .on_message("/trigger_letter", &|msg| {
            println!("LETTER TRIGGERRED");
            let letter = msg.args.get(0).unwrap();
            let as_char = letter.clone().string().unwrap().chars().min().unwrap();
            let let_res = Letter::from(as_char)
                .unwrap();
            let packet = states_handle.borrow().get_letter_packet(&let_res);
            handle.borrow_mut().send(packet);

        })
        .on_message("/modify_letter", &|msg| {
            let letter = msg.args.get(0).unwrap();
            let as_char = letter.clone().string().unwrap().chars().min().unwrap();
            let let_res = Letter::from(as_char)
                .unwrap();
            let mod_num = msg.args.get(1).unwrap().clone().int().unwrap();
            states_handle.borrow_mut().modify_number(&let_res, mod_num);
            let packet = states_handle.borrow().get_letter_packet(&let_res);
            handle.borrow_mut().send(packet);

        })
        .begin();

}
