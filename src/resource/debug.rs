use std::fmt;

use resource::resource::{Resource, to_json};

impl fmt::Debug for Resource {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", to_json(self))
    }
}
