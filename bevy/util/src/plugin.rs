use bevy::prelude::*;

use crate::asset::markdown_asset::{MarkDownAsset, MarkDownAssetLoader};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(MarkDownAssetLoader)
            .init_asset::<MarkDownAsset>();
    }
}
