use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub url: String,
    #[arg(short, long, default_value_t = String::from("root"))]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
}

pub fn str_to_vec(str: &str) -> Vec<&str> {
    str.split(',').map(|s| s.trim()).collect()
}

pub fn print_help() {
    println!("Welcome to the XAPI CLI!");
    println!("Please enter a method from the XAPI.");
    println!();
    println!(
        "If the method requires parameters, use the format: <method> [<param1>, <param2>, ...]"
    );
    println!("For example: VM.send_sysrq [OpaqueRef:023d46a3-ed95-489d-be64-6773ccd71477,b]");
    println!();
    println!(
        "You can filter the first level of results by using the notation {{<filter1, filter2>}}."
    );
    println!("For example: VM.get_record [OpaqueRef:023d46a3-ed95-489d-be64-6773ccd71477] {{name_label, description}}");
    println!("Or: VM.get_all_records {{OpaqueRef:5859a026-d48c-867e-9d02-d2e8937f9fd9}}");
    println!();
    println!("To quit, type 'exit'.");
    println!();
}

macro_rules! println_color {
    ($color:expr, $($arg:tt)*) => {{
        println!("\x1b[{}m{}\x1b[0m", $color, format!($($arg)*));
    }};
}

macro_rules! println_err {
    ($($arg:tt)*) => {
        println_color!(91, $($arg)*);
    };
}

macro_rules! println_success {
    ($($arg:tt)*) => {
        println_color!(92, $($arg)*);
    };
}

macro_rules! println_info {
    ($($arg:tt)*) => {
        println_color!(94, $($arg)*);
    };
}
