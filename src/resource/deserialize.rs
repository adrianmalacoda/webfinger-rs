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
        let mut links : Option<Vec<ResourceLink>> = None;

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

        let links : Vec<ResourceLink> = match links {
            Some(l) => l,
            None => Vec::new(),
        };

        let properties : HashMap<String, String> = match properties {
            Some(p) => p,
            None => HashMap::new(),
        };


        try!(visitor.end());

        Ok(Resource{subject: subject,
                    aliases: aliases,
                    properties: properties,
                    links: links
        })
    }
}



enum ResourceLinkField {
    Rel,
    Href,
    Type,
    Titles,
    Properties
}

impl serde::Deserialize for ResourceLinkField {
    fn deserialize<D>(deserializer: &mut D) -> Result<ResourceLinkField, D::Error>
        where D: serde::de::Deserializer
    {
        struct ResourceLinkFieldVisitor;

        impl serde::de::Visitor for ResourceLinkFieldVisitor {
            type Value = ResourceLinkField;

            fn visit_str<E>(&mut self, value: &str) -> Result<ResourceLinkField, E>
                where E: serde::de::Error
            {
                match value {
                    "rel" => Ok(ResourceLinkField::Rel),
                    "href" => Ok(ResourceLinkField::Href),
                    "type" => Ok(ResourceLinkField::Type),
                    "titles" => Ok(ResourceLinkField::Titles),
                    "properties" => Ok(ResourceLinkField::Properties),

                    _ => Err(serde::de::Error::syntax("expected “rel”, “href”, “type”, “titles”, or “properties”")),
                }
            }
        }

        deserializer.visit(ResourceLinkFieldVisitor)
    }
}


pub struct ResourceLinkVisitor;

impl serde::de::Visitor for ResourceLinkVisitor {
    type Value = ResourceLink;

    #[inline]
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<ResourceLink, V::Error>
        where V: serde::de::MapVisitor
    {


        let mut rel : Option<String> = None;
        let mut href : Option<String> = None;
        let mut type_ : Option<String> = None;
        let mut titles : Option<HashMap<String, String>> = None;
        let mut properties : Option<HashMap<String, String>> = None;

        loop {
            match try!(visitor.visit_key()){
                Some(ResourceLinkField::Rel) => rel = Some(try!(visitor.visit_value())),
                Some(ResourceLinkField::Href) => href = Some(try!(visitor.visit_value())),
                Some(ResourceLinkField::Type) => type_ = Some(try!(visitor.visit_value())),
                Some(ResourceLinkField::Titles) => titles = Some(try!(visitor.visit_value())),
                Some(ResourceLinkField::Properties) => properties = Some(try!(visitor.visit_value())),

                None => { break; },
            }
        }

        try!(visitor.end());

        let rel : String = rel.unwrap();
        let titles : HashMap<String, String> = match titles {
            Some(x) => x,
            None => HashMap::new()
        };
        let properties : HashMap<String, String> = match properties {
            Some(x) => x,
            None => HashMap::new()
        };

        Ok(ResourceLink{rel: rel,
                        href: href,
                        type_: type_,
                        titles: titles,
                        properties: properties
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






impl serde::de::Deserialize for ResourceLink {
    fn deserialize<D>(deserializer: &mut D) -> Result<ResourceLink, D::Error>
        where D: serde::de::Deserializer,
    {
        deserializer.visit(ResourceLinkVisitor)
    }
}

