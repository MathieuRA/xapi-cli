#[macro_export]
macro_rules! println_err {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        print!("\x1b[91m");
        println!($fmt $(, $arg)*);
        print!("\x1b[0m");
    }};
}

#[macro_export]
macro_rules! println_success {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        print!("\x1b[92m");
        println!($fmt $(, $arg)*);
        print!("\x1b[0m");
    }};
}

#[macro_export]
macro_rules! println_warn {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        print!("\x1b[93m");
        println!($fmt $(, $arg)*);
        print!("\x1b[0m");
    }};
}

#[macro_export]
macro_rules! println_info {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        print!("\x1b[94m");
        println!($fmt $(, $arg)*);
        print!("\x1b[0m");
    }};
}
