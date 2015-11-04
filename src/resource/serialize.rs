use serde;

use resource::resource::{Resource, ResourceLink};

impl serde::Serialize for ResourceLink {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.visit_struct("ResourceLink", ResourceLinkMapVisitor {
            value: self,
            state: 0,
        })
    }
}


struct ResourceLinkMapVisitor<'a> {
    value: &'a ResourceLink,
    state: u8,
}


impl<'a> serde::ser::MapVisitor for ResourceLinkMapVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: serde::Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("rel", &self.value.rel))))
            }
            1 => {
                self.state += 1;
                let href : Option<String> = self.value.href.clone();
                match href {
                    None => Ok(None),
                    Some(x) =>  Ok(Some(try!(serializer.visit_struct_elt("href", x))))
                }
            }
            2 => {
                self.state += 1;
                let type_ : Option<String> = self.value.type_.clone();
                match type_ {
                    None => Ok(None),
                    Some(x) => Ok(Some(try!(serializer.visit_struct_elt("type", x))))
                }
            }
            3 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("titles", &self.value.titles))))
            }
            4 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("properties", &self.value.properties))))
            }
            _ => {
                Ok(None)
            }
        }
    }
}


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
                Ok(Some(try!(serializer.visit_struct_elt("links", &self.value.links))))
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
