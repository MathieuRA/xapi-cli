extern crate xmlrpc;

use clap::Parser;
use std::{collections::HashMap, io};
use utils::{print_help, CliArgs};
use xapi::xapi::Xapi;

use crate::utils::str_to_vec;

mod r#macro;
mod utils;
mod xapi;

fn main() {
    let args = CliArgs::parse();

    let mut xapi = Xapi::new(args.url.clone(), args.username, args.password);
    xapi.connect_with_user_input();
    print_help();

    loop {
        println_info!("Enter a method:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            println_info!("Exiting the XAPI CLI. Goodbye!");
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
                println_warn!("Cannot filter result");
                println!("{:#?}", unwrapped_result);
            }
        }
    }
}
