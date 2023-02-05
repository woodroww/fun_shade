use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError}, mesh::MeshVertexBufferLayout},
pbr::{MaterialPipelineKey, MaterialPipeline},
};

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
//    #[uniform(0)]
}

impl Material for CoolMaterial {
    fn vertex_shader() -> ShaderRef {
       "my_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "my_frag.wgsl".into()
    }
    // this allows transparency
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout, // an entitys layout
        _key: MaterialPipelineKey<Self>, // an entitys key
    ) -> Result<(), SpecializedMeshPipelineError> {

        // this doesn't cull any faces
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "215519A9-0958-4EDF-A3FF-084C82232E06"]
pub struct GeometryMaterial {
//    #[uniform(0)]
}

impl Material for GeometryMaterial {
    fn vertex_shader() -> ShaderRef {
        "geo_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "geo_frag.wgsl".into()
    }
}
