extern crate xmlrpc;

use clap::Parser;
use std::io;
use utils::CliArgs;
use xapi::xapi::Xapi;

use crate::utils::str_to_vec;

mod utils;
mod xapi;

fn main() {
    let args = CliArgs::parse();

    let mut xapi = Xapi::new(
        String::from(args.url),
        String::from(args.username),
        String::from(args.password),
    );
    xapi.connect();

    println!("Welcome to the XAPI CLI!");
    println!("Please enter a method from the XAPI.");
    println!(
        "If the method requires parameters, use the format: <method> [<param1>, <param2>, ...]"
    );
    println!("For example: VM.send_sysrq [OpaqueRef:023d46a3-ed95-489d-be64-6773ccd71477,b]");
    println!("To quit, type 'exit'.");
    println!();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let _method = input.trim();

        if _method == "exit" {
            println!("Exiting the XAPI CLI. Goodbye!");
            break;
        } else {
            let mut split = _method.split(" ");
            let mut params = vec![];

            let method = split.next().unwrap();
            if let Some(value) = split.next() {
                println!("{value}");
                params = str_to_vec(value);
            }

            println!("{:#?}", xapi.call(method, params));
        }

        println!("Enter a method:");

    }
}
