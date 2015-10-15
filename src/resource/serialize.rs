use std::collections::HashMap;

use serde;

use resource::resource::{Resource, ResourceLink};

impl serde::Serialize for Resource {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.visit_struct("Resource", ResourceMapVisitor {
            value: self,
            state: 0,
        })
    }
}

struct ResourceMapVisitor<'a> {
    value: &'a Resource,
    state: u8,
}

fn links_to_hashmap(links: &Vec<ResourceLink>) -> Vec<HashMap<String, String>> {
    let maps : Vec<HashMap<String, String>> = Vec::new();

    // @TODO Complete

    maps
}

impl<'a> serde::ser::MapVisitor for ResourceMapVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: serde::Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("aliases", &self.value.aliases))))
            }
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("links", links_to_hashmap(&self.value.links)))))
            }
            2 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("properties", &self.value.properties))))
            }
            3 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("subject", &self.value.subject))))
            }
            _ => {
                Ok(None)
            }
        }
    }
}
