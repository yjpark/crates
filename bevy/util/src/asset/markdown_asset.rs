use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::{TypeUuid, TypePath};
use bevy::utils::BoxedFuture;

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "ae3cb724-f08b-4ceb-a7dd-c7d6781ba49b"]
pub struct MarkDownAsset {
    pub text: String,
}

impl From<String> for MarkDownAsset {
    fn from(v: String) -> Self {
        Self { text: v }
    }
}

impl From<&str> for MarkDownAsset {
    fn from(v: &str) -> Self {
        Self {
            text: String::from(v),
        }
    }
}

#[derive(Default)]
pub struct MarkDownAssetLoader;

pub type LoadError = bevy::asset::Error;
pub type LoadResult = std::result::Result<(), LoadError>;

impl AssetLoader for MarkDownAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, LoadResult> {
        Box::pin(async move {
            let text = String::from_utf8(bytes.to_vec())?;
            load_context.set_default_asset(LoadedAsset::new(MarkDownAsset::from(text)));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["md"]
    }
}
