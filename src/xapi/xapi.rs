extern crate xmlrpc;
use xmlrpc::Request;
use xmlrpc::Value;

pub struct Xapi {
    url: String,
    username: String,
    password: String,

    session_ref: Option<String>,
}

impl Xapi {
    fn get_full_url(&self) -> String {
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

        let response = request.call_url(self.get_full_url()).expect(
            format!(
                "An error has occurred during the API call using the method: {}",
                method
            )
            .as_str(),
        );

        if let Some(result) = response.get("Value") {
            Ok(result.clone())
        } else {
            Err(response)
        }
    }

    pub fn call(&self, method: &str, params: Vec<&str>) -> Result<Value, Value> {
        self._call(method, params, false)
    }

    pub fn connect(&mut self) -> () {
        let params = vec![self.username.as_str(), self.password.as_str()];
        let session_ref = self
            ._call("session.login_with_password", params, true)
            .expect("An error as occured during: 'session.login_with_password'");

        self.session_ref = Some(session_ref.as_str().unwrap().to_string());
    }

    pub fn new(url: String, username: String, password: String) -> Xapi {
        Xapi {
            url,
            username,
            password,

            session_ref: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.session_ref.is_some()
    }
}
