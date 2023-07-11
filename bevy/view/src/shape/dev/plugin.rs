use crate::prelude::*;
use bevy::prelude::*;

use super::theme::ViewShapeDevTheme;

#[derive(Debug, Default)]
pub struct ViewShapeDevPlugin;

impl Plugin for ViewShapeDevPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ViewShapeDevTheme>();
        app.add_system(on_add_layout_data);
        app.add_system(on_layout_data_changed);
    }
}

fn on_add_layout_data(
    mut commands: Commands,
    theme: Res<ViewShapeDevTheme>,
    layout_query: Query<(Entity, &LayoutData), Added<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        layout.create(&mut commands, &theme, entity);
    }
}

fn on_layout_data_changed(
    mut commands: Commands,
    theme: Res<ViewShapeDevTheme>,
    layout_query: Query<(Entity, &LayoutData), Changed<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        layout.update(&mut commands, &theme, entity);
    }
}
