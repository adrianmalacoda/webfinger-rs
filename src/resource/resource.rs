use std::collections::hash_map::HashMap;
use std::option::Option;

#[derive(Serialize, Deserialize)]
pub struct Resource {

    // The value of the "subject" member is a URI that identifies the entity
    // that the JRD describes.
    pub subject: String,


    // The "aliases" array is an array of zero or more URI strings that
    // identify the same entity as the "subject" URI.
    #[serde(default = "Vec::new")]
    pub aliases: Vec<String>,


    // The "properties" object comprises zero or more name/value pairs whose
    // names are URIs (referred to as "property identifiers") and whose
    // values are strings or null. Properties are used to convey additional
    // information about the subject of the JRD.
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, String>,


    // The "links" array has any number of member objects, each of which
    // represents a link [4].
    #[serde(default = "Vec::new")]
    pub links: Vec<ResourceLink>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceLink {
    // Each of these link objects can have the following members:
    //         o rel
    //         o href
    //         o type
    //         o titles
    //         o properties

    // The value of the "rel" member is a string that is either a URI or a
    // registered relation ‘type’ (see RFC 5988).  The value of the
    // "rel" member MUST contain exactly one URI or registered relation
    // type.  The URI or registered relation type identifies the type of the
    // link relation.
    // The "rel" member MUST be present in the link relation object.
    pub rel: String,


    // The value of the "type" member is a string that indicates the media
    // type of the target resource (see RFC 6838).
    #[serde(rename = "type")]
    pub type_: Option<String>,


    // The value of the "href" member is a string that contains a URI
    // pointing to the target resource.
    pub href: Option<String>,


    // The "titles" object comprises zero or more name/value pairs whose
    // names are a language tag [11] or the string "und".  The string is
    // human-readable and describes the link relation.  More than one title
    // for the link relation MAY be provided for the benefit of users who
    // utilize the link relation, and, if used, a language identifier SHOULD
    // be duly used as the name.  If the language is unknown or unspecified,
    // then the name is "und".
    #[serde(default = "HashMap::new")]
    pub titles: HashMap<String, String>,


    // The "properties" object within the link relation object comprises
    // zero or more name/value pairs whose names are URIs (referred to as
    // "property identifiers") and whose values are strings or null.
    // Properties are used to convey additional information about the link
    // relation.
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, String>,
}
