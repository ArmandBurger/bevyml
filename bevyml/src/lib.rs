use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bevyml_parser::BevymlParser;
pub use bevyml_parser::inode::BevyNodeTree;
use std::{error::Error, fmt, str};

#[derive(Asset, TypePath, Debug)]
pub struct BevymlAsset {
    pub roots: Vec<BevyNodeTree>,
}

#[derive(Default)]
pub struct BevymlAssetLoader;

#[non_exhaustive]
#[derive(Debug)]
pub enum BevymlAssetLoaderError {
    Io(std::io::Error),
    Utf8(str::Utf8Error),
    Parse(bevyml_parser::itree::ITreeError),
}

impl fmt::Display for BevymlAssetLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "could not load asset: {err}"),
            Self::Utf8(err) => write!(f, "invalid utf-8 in asset: {err}"),
            Self::Parse(err) => write!(f, "could not parse bevyml: {err}"),
        }
    }
}

impl Error for BevymlAssetLoaderError {}

impl From<std::io::Error> for BevymlAssetLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<str::Utf8Error> for BevymlAssetLoaderError {
    fn from(value: str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<bevyml_parser::itree::ITreeError> for BevymlAssetLoaderError {
    fn from(value: bevyml_parser::itree::ITreeError) -> Self {
        Self::Parse(value)
    }
}

impl AssetLoader for BevymlAssetLoader {
    type Asset = BevymlAsset;
    type Settings = ();
    type Error = BevymlAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let source = str::from_utf8(&bytes)?;
        let mut parser = BevymlParser::new();
        let tree = parser.parse(source)?;
        Ok(BevymlAsset { roots: tree.into() })
    }

    fn extensions(&self) -> &[&str] {
        &["bevyml", "html"]
    }
}

pub struct BevymlAssetPlugin;

impl Plugin for BevymlAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BevymlAsset>()
            .init_asset_loader::<BevymlAssetLoader>();
    }
}
