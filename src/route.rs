use std::ops::{Deref, DerefMut};

pub struct RouteBuilder {
    span: Vec<String>,
    delim: Delimiter,
}

impl Deref for RouteBuilder {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.span
    }
}

impl DerefMut for RouteBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.span
    }
}

enum Delimiter {
    /// " -> "
    Arrow,
}

impl RouteBuilder {
    pub const fn with_arrow() -> Self {
        Self {
            span: Vec::new(),
            delim: Delimiter::Arrow,
        }
    }

    pub fn build(self) -> String {
        let mut res = String::new();
        let len = self.span.len();
        for (idx, item) in self.span.into_iter().enumerate() {
            if idx < len - 1 {
                res.extend([
                    item.as_str(),
                    match self.delim {
                        Delimiter::Arrow => " -> ",
                    },
                ])
            } else {
                res.push_str(&item);
            }
        }
        res
    }
}

#[test]
fn test_route_builder() {
    let mut builder = RouteBuilder::with_arrow();
    builder.push("Apple".to_owned());
    builder.push("Banana".to_owned());
    builder.push("Orange".to_owned());
    builder.push("Peach".to_owned());

    assert_eq!(
        "Apple -> Banana -> Orange -> Peach",
        builder.build().as_str()
    );
}
