use resource::resource::Resource;

use serde_json;
use std::fmt;

impl fmt::Display for Resource {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", match serde_json::to_string(self) {
            Ok(json) => json,
            Err(err) => format!("<Failed to serialize result to JSON: {:?}>", err)
        })
    }
}
