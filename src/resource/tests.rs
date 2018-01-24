use serde_json;

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use resource::resource::{Resource, ResourceLink};

// Serialized resources for reference
#[allow(dead_code)]
const MINIMAL_TEST_STR : &'static str =
    "{\"aliases\":[],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}";

#[allow(dead_code)]
const ALIASES_TEST_STR : &'static str =
    "{\"aliases\":[\"alias1\",\"alias2\"],\"links\":[],\"properties\":{},\"subject\":\"Subject\"}";

#[allow(dead_code)]
const PROPERTIES_TEST_STR : &'static str =
    "{\"aliases\":[],\"links\":[],\"properties\":{\"http://prop\":\"val\"},\"subject\":\"Subject\"}";

#[allow(dead_code)]
const SIMPLE_LINK_TEST_STR : &'static str =
    "{\"aliases\":[],\"links\":[{\"rel\":\"example\"}],\"properties\":{},\"subject\":\"Subject\"}";

#[allow(dead_code)]
const FULL_LINK_TEST_STR : &'static str =
    "{\"aliases\":[],\"links\":[{\"rel\":\"example\",\"href\":\"http://sample/\",\"type\":\"sample\",\"titles\":{\"es\":\"ejemplo\"},\"properties\":{\"sample_type\":\"test\"}}],\"properties\":{},\"subject\":\"Subject\"}";

#[allow(dead_code)]
const FULL_RESOURCE_TEST_STR : &'static str =
    "{\"aliases\":[\"one\",\"two\"],\"links\":[{\"rel\":\"example\",\"href\":\"http://sample/\",\"type\":\"sample\",\"titles\":{\"es\":\"ejemplo\"},\"properties\":{\"sample_type\":\"test\"}}],\"properties\":{\"http://prop\":\"val\"},\"subject\":\"Subject\"}";


#[test]
#[ignore]
fn minimal_serialize_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let links : Vec<ResourceLink> = Vec::new();

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, MINIMAL_TEST_STR.to_string());
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

    let deserialized_resource: Resource = serde_json::from_str(&MINIMAL_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
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


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, ALIASES_TEST_STR.to_string());
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

    let deserialized_resource: Resource = serde_json::from_str(&ALIASES_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
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


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, PROPERTIES_TEST_STR.to_string());
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

    let deserialized_resource: Resource = serde_json::from_str(&PROPERTIES_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
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


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, SIMPLE_LINK_TEST_STR.to_string());
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

    let deserialized_resource: Resource = serde_json::from_str(&SIMPLE_LINK_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
fn serialize_with_full_link_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let mut links : Vec<ResourceLink> = Vec::new();

    let mut titles : HashMap<String, String> = HashMap::new();
    titles.insert("es".to_string(), "ejemplo".to_string());

    let mut link_properties : HashMap<String, String> = HashMap::new();
    link_properties.insert("sample_type".to_string(), "test".to_string());


    links.push(ResourceLink{rel: "example".to_string(),
                            type_: Some("sample".to_string()),
                            href: Some("http://sample/".to_string()),
                            titles: titles,
                            properties: link_properties});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, FULL_LINK_TEST_STR);
}


#[test]
fn deserialize_with_full_link_test(){
    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = Vec::new();
    let properties : HashMap<String, String> = HashMap::new();
    let mut links : Vec<ResourceLink> = Vec::new();

    let mut titles : HashMap<String, String> = HashMap::new();
    titles.insert("es".to_string(), "ejemplo".to_string());

    let mut link_properties : HashMap<String, String> = HashMap::new();
    link_properties.insert("sample_type".to_string(), "test".to_string());

    links.push(ResourceLink{rel: "example".to_string(),
                            type_: Some("sample".to_string()),
                            href: Some("http://sample/".to_string()),
                            titles: titles,
                            properties: link_properties});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let deserialized_resource: Resource = serde_json::from_str(&FULL_LINK_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
fn serialize_full_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = vec!("one".to_string(), "two".to_string());

    let mut properties : HashMap<String, String> = HashMap::new();
    properties.insert("http://prop".to_string(), "val".to_string());

    let mut links : Vec<ResourceLink> = Vec::new();
    let mut titles : HashMap<String, String> = HashMap::new();
    titles.insert("es".to_string(), "ejemplo".to_string());

    let mut link_properties : HashMap<String, String> = HashMap::new();
    link_properties.insert("sample_type".to_string(), "test".to_string());


    links.push(ResourceLink{rel: "example".to_string(),
                            type_: Some("sample".to_string()),
                            href: Some("http://sample/".to_string()),
                            titles: titles,
                            properties: link_properties});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};


    let r : String = serde_json::to_string(&sample_resource).unwrap();
    assert_eq!(r, FULL_RESOURCE_TEST_STR);
}


#[test]
fn deserialize_full_test(){
    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = vec!("one".to_string(), "two".to_string());

    let mut properties : HashMap<String, String> = HashMap::new();
    properties.insert("http://prop".to_string(), "val".to_string());

    let mut links : Vec<ResourceLink> = Vec::new();
    let mut titles : HashMap<String, String> = HashMap::new();
    titles.insert("es".to_string(), "ejemplo".to_string());

    let mut link_properties : HashMap<String, String> = HashMap::new();
    link_properties.insert("sample_type".to_string(), "test".to_string());


    links.push(ResourceLink{rel: "example".to_string(),
                            type_: Some("sample".to_string()),
                            href: Some("http://sample/".to_string()),
                            titles: titles,
                            properties: link_properties});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let deserialized_resource: Resource = serde_json::from_str(&FULL_RESOURCE_TEST_STR.to_string()).unwrap();

    assert_eq!(sample_resource, deserialized_resource);
}


#[test]
#[ignore]
fn display_test(){

    let subject : String = "Subject".to_string();
    let aliases : Vec<String> = vec!("one".to_string(), "two".to_string());

    let mut properties : HashMap<String, String> = HashMap::new();
    properties.insert("http://prop".to_string(), "val".to_string());

    let mut links : Vec<ResourceLink> = Vec::new();
    let mut titles : HashMap<String, String> = HashMap::new();
    titles.insert("es".to_string(), "ejemplo".to_string());

    let mut link_properties : HashMap<String, String> = HashMap::new();
    link_properties.insert("sample_type".to_string(), "test".to_string());


    links.push(ResourceLink{rel: "example".to_string(),
                            type_: Some("sample".to_string()),
                            href: Some("http://sample/".to_string()),
                            titles: titles,
                            properties: link_properties});

    let sample_resource : Resource = Resource{subject: subject,
                                              aliases: aliases,
                                              properties: properties,
                                              links: links};

    let serialized_resource: String = format!("{}", sample_resource);

    assert_eq!(FULL_RESOURCE_TEST_STR.to_string(), serialized_resource);
}
