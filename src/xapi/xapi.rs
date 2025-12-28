extern crate xmlrpc;
use std::io;

use xmlrpc::Request;
use xmlrpc::Value;

pub struct Xapi {
    url: String,
    username: String,
    password: String,

    session_ref: Option<String>,
}

impl Xapi {
    pub fn get_full_url(&self) -> String {
        format!("{}/{}", self.url, "xmlrpc")
    }

    fn _call(&self, method: &str, params: Vec<&str>, as_anonymous: bool) -> Result<Value, Value> {
        let mut request = Request::new(method);

        if !as_anonymous {
            if self.is_connected() {
                request = request.arg(self.session_ref.clone());
            } else {
                return Err(Value::String(String::from("Not connected to the XAPI")));
            }
        }

        for param in params {
            request = request.arg(param);
        }

        let response = request.call_url(self.get_full_url());

        if response.is_err() {
            return Err(Value::String(response.err().unwrap().to_string()));
        }

        let resp = response.ok().unwrap();
        if let Some(result) = resp.get("Value") {
            Ok(result.clone())
        } else {
            Err(resp)
        }
    }

    pub fn call(&self, method: &str, params: Vec<&str>) -> Result<Value, Value> {
        self._call(method, params, false)
    }

    fn connect(&mut self) -> Result<(), Value> {
        let params = vec![self.username.as_str(), self.password.as_str()];
        let session_ref = self._call("session.login_with_password", params, true);

        if let Ok(session) = session_ref {
            self.session_ref = Some(session.as_str().unwrap().to_string());
            return Ok(());
        }

        Err(session_ref.err().unwrap())
    }

    pub fn connect_with_user_input(&mut self) {
        while !self.is_connected() {
            let connect_result = self.connect();
            if connect_result.is_ok() {
                return;
            }

            println!("Failed to connect to the XAPI: {}", self.get_full_url());
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

            self.username = username_input.trim().to_string();
            self.password = password_input.trim().to_string();
        }
    }

    pub fn new(url: String, username: String, password: String) -> Xapi {
        Xapi {
            url,
            username,
            password,

            session_ref: None,
        }
    }

    fn is_connected(&self) -> bool {
        self.session_ref.is_some()
    }
}
