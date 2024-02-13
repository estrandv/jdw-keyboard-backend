mod state_model;

use jdw_osc_lib::osc_stack::OSCStack;

fn main() {

    let payloads =

    OSCStack::init("127.0.0.1:13458".to_string())
        .on_message("/hello", &|msg| {println!("HE");})
        .begin();

}
