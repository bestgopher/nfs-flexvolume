use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Serialize, Serializer};

pub enum Status {
    Success,
    Failure,
    NotSupported,
}

impl Default for Status {
    fn default() -> Self {
        Self::Success
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(
            match self {
                Self::Failure => "Failure",
                Self::Success => "Success",
                Self::NotSupported => "Not supported"
            }
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).map_err(|_| fmt::Error)?.as_str())
    }
}


#[derive(Default, Serialize)]
pub struct Capabilities {
    attach: bool,
}

#[derive(Default, Serialize)]
pub struct Response {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Capabilities>,
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn with_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_attach(mut self, attach: bool) -> Self {
        self.capabilities = Some(Capabilities { attach });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let r = Response::new().with_status(Status::NotSupported);
        assert_eq!(
            r#"{"status":"Not supported"}"#,
            serde_json::to_string(&r).unwrap()
        );

        let r = r.with_attach(false);
        assert_eq!(
            r#"{"status":"Not supported","capabilities":{"attach":false}}"#,
            serde_json::to_string(&r).unwrap()
        );

        let r = r.with_message("hello");
        assert_eq!(
            r#"{"status":"Not supported","message":"hello","capabilities":{"attach":false}}"#,
            serde_json::to_string(&r).unwrap()
        );
    }
}