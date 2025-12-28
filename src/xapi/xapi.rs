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

    pub fn connect(&mut self) -> Result<(), Value> {
        let params = vec![self.username.as_str(), self.password.as_str()];
        let session_ref = self._call("session.login_with_password", params, true);

        if let Ok(session) = session_ref {
            self.session_ref = Some(session.as_str().unwrap().to_string());
            return Ok(());
        }

        Err(session_ref.err().unwrap())
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
