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
    let cleaned_input = str.trim_start_matches('[').trim_end_matches(']');
    cleaned_input.split(',').map(|s| s.trim()).collect()
}
