use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

// bevy/crates/bevy_render/src/mesh/shape
// https://answers.unity.com/questions/1850185/mesh-triangles-not-filling-whole-space-2.html

pub struct SubdividedPlane {
    pub x_vertex_count: u32,
    pub z_vertex_count: u32,
}

impl From<SubdividedPlane> for Mesh {
    fn from(plane: SubdividedPlane) -> Self {
        let num_vertices = (plane.z_vertex_count * plane.x_vertex_count) as usize;
        let num_indices = ((plane.z_vertex_count - 1) * (plane.x_vertex_count - 1) * 6) as usize;
        let up = Vec3::Y.to_array();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        for y in 0..plane.z_vertex_count {
            for x in 0..plane.x_vertex_count {
                let tx = x as f32 / (plane.x_vertex_count - 1) as f32;
                let ty = y as f32 / (plane.z_vertex_count - 1) as f32;
                positions.push([-0.5 + tx, 0.0, -0.5 + ty]);
                normals.push(up);
                uvs.push([tx, 1.0 - ty]);
            }
        }

        for y in 0..plane.z_vertex_count - 1 {
            for x in 0..plane.x_vertex_count - 1 {
                let quad = y * plane.x_vertex_count + x;
                indices.push(quad + plane.x_vertex_count + 1);
                indices.push(quad + 1);
                indices.push(quad + plane.x_vertex_count);
                indices.push(quad);
                indices.push(quad + plane.x_vertex_count);
                indices.push(quad + 1);
            }
        }

        assert_eq!(num_vertices, positions.len());
        assert_eq!(num_vertices, normals.len());
        assert_eq!(num_vertices, uvs.len());
        assert_eq!(num_indices, indices.len());

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

