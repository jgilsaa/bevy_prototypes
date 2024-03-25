use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

pub struct GridMesh {
    pub width: usize,
    pub height: usize,
    pub tile_size: f32,
}

impl From<GridMesh> for Mesh {
    fn from(grid: GridMesh) -> Self {
        let tile_size = grid.tile_size as f32;
        let x_end = grid.width as f32 * tile_size;
        let z_end = grid.height as f32 * tile_size;

        let mut vertices = Vec::new();

        // Vertical lines
        for x in 0..grid.width {
            let x = x as f32;
            vertices.push(Vec3::new(x * tile_size, 0.0, 0.0));
            vertices.push(Vec3::new(x * tile_size, 0.0, z_end));
        }

        // Horizontal lines
        for z in 0..grid.height {
            let z = z as f32;
            vertices.push(Vec3::new(0.0, 0.0, z * tile_size));
            vertices.push(Vec3::new(x_end, 0.0, z * tile_size));
        }

        // Add right end line
        vertices.push(Vec3::new(x_end, 0.0, 0.0));
        vertices.push(Vec3::new(x_end, 0.0, z_end));

        // Add bottom end line
        vertices.push(Vec3::new(0.0, 0.0, z_end));
        vertices.push(Vec3::new(x_end, 0.0, z_end));

        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}
