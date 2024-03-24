use bevy::prelude::*;

mod grid_mesh;
use grid_mesh::GridMesh;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_grid_mesh);
    }
}

fn draw_grid_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let width = 4;
    let height = 4;

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(GridMesh {
            width,
            height,
            tile_size: 2.0,
        }),
        material: materials.add(Color::YELLOW),
        // Make grid center aligned
        transform: Transform::from_xyz(-(width as f32), 0.0, -(height as f32)),
        ..default()
    });
}
