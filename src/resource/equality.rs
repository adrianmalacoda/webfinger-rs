use resource::resource::{Resource, ResourceLink};


impl Eq for Resource {}

impl PartialEq<Resource> for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.subject == other.subject &&
            self.aliases == other.aliases &&
            self.properties == other.properties &&
            self.links == other.links
    }

    fn ne(&self, other: &Resource) -> bool {
        !self.eq(other)
    }
}


impl Eq for ResourceLink {}

impl PartialEq<ResourceLink> for ResourceLink {
    fn eq(&self, other: &ResourceLink) -> bool {
        self.rel == other.rel &&
            self.href == other.href &&
            self.type_ == other.type_ &&
            self.properties == other.properties
    }

    fn ne(&self, other: &ResourceLink) -> bool {
        !self.eq(other)
    }
}

