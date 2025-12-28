extern crate xmlrpc;

use clap::Parser;
use std::{collections::HashMap, io};
use utils::{print_help, CliArgs};
use xapi::xapi::Xapi;

use crate::utils::str_to_vec;

mod utils;
mod xapi;

fn main() {
    let args = CliArgs::parse();

    let mut xapi = Xapi::new(args.url.clone(), args.username, args.password);

    while !xapi.is_connected() {
        println!("inside the loop!");
        let connect_result = xapi.connect();
        if connect_result.is_err() {
            println!("Failed to connect to the XAPI: {}", xapi.get_full_url());
            println!("{:?}", connect_result.err().unwrap());

            let mut username_input = String::new();
            println!("Please provide a valid username");
            io::stdin()
                .read_line(&mut username_input)
                .expect("Failed to read username");

            let mut password_input = String::new();
            println!("Please provide a valid password");
            io::stdin()
                .read_line(&mut password_input)
                .expect("Failed to read password");

            xapi = Xapi::new(
                args.url.clone(),
                String::from(username_input.trim()),
                String::from(password_input.trim()),
            );
        }
    }

    print_help();

    loop {
        println!("Enter a method:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            println!("Exiting the XAPI CLI. Goodbye!");
            break;
        }

        if trimmed_input == "help" {
            print_help();
            continue;
        }

        let mut split = trimmed_input.split(" ");

        let mut params = vec![];
        let mut filters = vec![];

        let method = split.next().unwrap();

        for mut value in split {
            value = value.trim();
            let inner_value = &value[1..value.len() - 1];

            match value.chars().next() {
                Some('[') => params = str_to_vec(inner_value),
                Some('{') => filters = str_to_vec(inner_value),
                _ => println!("Unknown param: {}", value),
            }
        }

        let call_result = xapi.call(method, params);

        if filters.is_empty() || call_result.is_err() {
            println!("{:#?}", call_result);
            continue;
        }

        let unwrapped_result = call_result.unwrap();
        let response_struct = unwrapped_result.as_struct();

        match response_struct {
            Some(resp) => {
                let mut filtered_results = HashMap::new();

                for filter in filters {
                    filtered_results.insert(filter, resp.get(filter));
                }
                println!("{:#?}", filtered_results);
            }
            _ => {
                println!("Cannot filter result");
                println!("{:#?}", unwrapped_result);
            }
        }
    }
}
