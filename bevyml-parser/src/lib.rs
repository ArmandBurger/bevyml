pub use tree_sitter;

pub mod inode;
pub mod inode_info;
pub mod itree;
use tree_sitter::Tree;

use bevy_derive::{Deref, DerefMut};
use std::{fs as blocking_fs, io, path::Path};
use tokio::fs as tokio_fs;
use tree_sitter::{LanguageError, Parser};

use crate::itree::{ITree, ITreeError};

#[derive(Deref, DerefMut)]
pub struct BevymlParser(Parser);

impl BevymlParser {
    /// Attempts to create a parser bound to the Bevyml language so the caller can deal with any
    /// `LanguageError` that shows up.
    pub fn try_new() -> Result<Self, LanguageError> {
        let mut parser = Self(Parser::new());
        parser.set_language(&tree_sitter_bevyml::LANGUAGE.into())?;

        Ok(parser)
    }

    /// Builds the parser for Bevyml and panics if loading the grammar fails; might go KABOOOM! xD
    pub fn new() -> Self {
        Self::try_new().expect("Error loading Bevyml grammar.")
    }

    pub fn parse<'source>(&mut self, txt: &'source str) -> Result<ITree<'source>, ITreeError> {
        let tree = self
            .0
            .parse(txt, None)
            .ok_or(ITreeError::MissingParseTree)?;
        ITree::try_from((&tree, txt))
    }

    /// Parses the contents of a file asynchronously using Tokio-backed file I/O.
    pub async fn parse_file<P>(&mut self, path: P) -> io::Result<Option<Tree>>
    where
        P: AsRef<Path>,
    {
        let source = tokio_fs::read_to_string(path.as_ref()).await?;
        Ok(self.0.parse(&source, None))
    }

    /// Parses the contents of a file with blocking I/O via Pollster so callers that do not run on an
    /// async runtime can still reuse the parser.
    pub fn parse_file_block_on<P>(&mut self, path: P) -> io::Result<Option<Tree>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_owned();

        pollster::block_on(async {
            let source = blocking_fs::read_to_string(&path)?;
            Ok(self.0.parse(&source, None))
        })
    }
}
