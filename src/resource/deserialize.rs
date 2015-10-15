use std::collections::HashMap;

use serde;

use resource::resource::{Resource, ResourceLink};


enum ResourceField {
    Subject,
    Aliases,
    Properties,
    Links
}

impl serde::Deserialize for ResourceField {
    fn deserialize<D>(deserializer: &mut D) -> Result<ResourceField, D::Error>
        where D: serde::de::Deserializer
    {
        struct ResourceFieldVisitor;

        impl serde::de::Visitor for ResourceFieldVisitor {
            type Value = ResourceField;

            fn visit_str<E>(&mut self, value: &str) -> Result<ResourceField, E>
                where E: serde::de::Error
            {
                match value {
                    "subject" => Ok(ResourceField::Subject),
                    "aliases" => Ok(ResourceField::Aliases),
                    "properties" => Ok(ResourceField::Properties),
                    "links" => Ok(ResourceField::Links),
                    _ => Err(serde::de::Error::syntax("expected “subject”, “aliases”, “properties”, or “links”")),
                }
            }
        }

        deserializer.visit(ResourceFieldVisitor)
    }
}

pub struct ResourceVisitor;

impl serde::de::Visitor for ResourceVisitor {
    type Value = Resource;

    #[inline]
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Resource, V::Error>
              where V: serde::de::MapVisitor
    {


        let mut subject : Option<String> = None;
        let mut aliases : Option<Vec<String>> = None;
        let mut properties : Option<HashMap<String, String>> = None;
        let mut links : Option<Vec<HashMap<String, String>>> = None;

        loop {
            match try!(visitor.visit_key()){
                Some(ResourceField::Subject) => subject = Some(try!(visitor.visit_value())),
                Some(ResourceField::Aliases) => aliases = Some(try!(visitor.visit_value())),
                Some(ResourceField::Properties) => properties = Some(try!(visitor.visit_value())),
                Some(ResourceField::Links) => links = Some(try!(visitor.visit_value())),

                None => { break; },
            }
        }


        let subject : String = match subject {
            Some(s) => s,
            None => try!(visitor.missing_field("subject")),
        };

        let aliases : Vec<String> = match aliases {
            Some(a) => a,
            None => Vec::new(),
        };

        let links : Vec<HashMap<String, String>> = match links {
            Some(l) => l,
            None => Vec::new(),
        };

        let properties : HashMap<String, String> = match properties {
            Some(p) => p,
            None => HashMap::new(),
        };


        try!(visitor.end());
        let linksn : Vec<ResourceLink> = Vec::new();

        Ok(Resource{subject: subject,
                    aliases: aliases,
                    properties: properties,
                    links: linksn
        })
    }
}


impl serde::de::Deserialize for Resource {
    fn deserialize<D>(deserializer: &mut D) -> Result<Resource, D::Error>
        where D: serde::de::Deserializer,
    {
        deserializer.visit(ResourceVisitor)
    }
}

