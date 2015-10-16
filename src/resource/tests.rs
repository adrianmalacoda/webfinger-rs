#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use resource::resource::{Resource, ResourceLink, to_json, from_json};

#[test]
fn minimal_serialize_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = to_json(&sample_resource);
    assert_eq!(r, "{\"aliases\":[],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}");
}


#[test]
fn minimal_deserialize_test(){
    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let r : String = "{\"aliases\":[],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}".to_string();
    let deserialized_resource: Resource = from_json(&r);

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
fn serialize_with_aliases_test(){

    let subject : String = "Subject".to_string();
    let mut aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    aliases.push("alias1".to_string());
    aliases.push("alias2".to_string());

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = to_json(&sample_resource);
    assert_eq!(r, "{\"aliases\":[\"alias1\",\"alias2\"],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}");
}


#[test]
fn deserialize_with_alias_test(){
    let subject : String = "Subject".to_string();
    let mut aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    aliases.push("alias1".to_string());
    aliases.push("alias2".to_string());

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let r : String = "{\"aliases\":[\"alias1\",\"alias2\"],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}".to_string();
    let deserialized_resource: Resource = from_json(&r);

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
fn serialize_with_properties_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let mut properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    properties.insert("http://prop".to_string(), "val".to_string());

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = to_json(&sample_resource);
    assert_eq!(r,
               "{\"aliases\":[],\"links\":[],\"properties\":{\"http://prop\":\"val\"},\"subject\":\"Subject\"}");
}


#[test]
fn deserialize_with_properties_test(){
    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let mut properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    properties.insert("http://prop".to_string(), "val".to_string());

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let r : String = "{\"aliases\":[],\"links\":[],\"properties\":{\"http://prop\":\"val\"},\"subject\":\"Subject\"}".to_string();
    let deserialized_resource: Resource = from_json(&r);

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
fn serialize_with_simple_link_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let mut links : Vec<ResourceLink> = Vec::new();

    links.push(ResourceLink{rel: "example".to_string(),
                            type_: None,
                            href: None,
                            titles: HashMap::new(),
                            properties: HashMap::new()});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = to_json(&sample_resource);
    assert_eq!(r,
               "{\"aliases\":[],\"links\":[{\"rel\":\"example\",\"href\":null,\"type\":null,\"titles\":{},\"properties\":{}}],\"properties\":{},\"subject\":\"Subject\"}");
}


#[test]
fn deserialize_with_simple_link_test(){
    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let mut links : Vec<ResourceLink> = Vec::new();

    links.push(ResourceLink{rel: "example".to_string(),
                            type_: None,
                            href: None,
                            titles: HashMap::new(),
                            properties: HashMap::new()});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let r : String = "{\"aliases\":[],\"links\":[{\"rel\":\"example\",\"titles\":{},\"properties\":{}}],\"properties\":{},\"subject\":\"Subject\"}".to_string();
    let deserialized_resource: Resource = from_json(&r);

    assert_eq!(sample_resource, deserialized_resource);
}
