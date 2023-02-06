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
       "shaders/my_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/my_frag.wgsl".into()
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
        "shaders/geo_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/geo_frag.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "5F9B8800-B148-487B-B43F-50CC36CB8114"]
pub struct JammyMaterial {
//    #[uniform(0)]
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl Material for JammyMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tex_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/tex_frag.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "5F9B8800-B148-487B-B43F-50CC36CB8114"]
pub struct GLSLMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
    pub alpha_mode: AlphaMode,
}

impl Material for GLSLMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tex.vert".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/tex.frag".into()
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout, // an entitys layout
        _key: MaterialPipelineKey<Self>, // an entitys key
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
