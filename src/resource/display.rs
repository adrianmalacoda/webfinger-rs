use resource::resource::{Resource, to_json};

use std::fmt;

impl fmt::Display for Resource {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", to_json(self))
    }
}
