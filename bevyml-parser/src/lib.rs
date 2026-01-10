use bevy_derive::{Deref, DerefMut};
use tree_sitter::Parser;

#[derive(Deref, DerefMut)]
pub struct BevymlParser(Parser);

impl BevymlParser {
    pub fn new() -> Self {
        let mut parser = Self(Parser::new());
        parser
            .set_language(&tree_sitter_bevyml::LANGUAGE.into())
            .expect("Error loading Bevyml grammar.");

        parser
    }
}
