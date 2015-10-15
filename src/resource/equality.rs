use resource::resource::Resource;

impl Eq for Resource {}

impl PartialEq<Resource> for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.subject == other.subject &&
            self.aliases == other.aliases &&
            self.properties == other.properties // &&
            // self.links == other.links
    }

    fn ne(&self, other: &Resource) -> bool {
        !self.eq(other)
    }
}

