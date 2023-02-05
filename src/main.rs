use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError}, mesh::MeshVertexBufferLayout},
    window::PresentMode, pbr::{MaterialPipelineKey, MaterialPipeline},
};

mod camera;
mod cylinder;

use bevy_mod_picking::{PickingCameraBundle, PickableBundle, DefaultPickingPlugins};
use camera::{CameraPlugin, PanOrbitCamera};

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cylinder::Cylinder;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const WIDTH: f32 = 900.0;
//pub const RESOLUTION: f32 = 16.0 / 9.0;

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
        pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout, // an entitys layout
        key: MaterialPipelineKey<Self>, // an entitys key
    ) -> Result<(), SpecializedMeshPipelineError> {

        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

fn main() {

    let width = 1290.0;
    let height = 700.0;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width,
                        height,
                        title: "Bevy Material Tutorial".to_string(),
                        present_mode: PresentMode::Fifo,
                        position: WindowPosition::At(Vec2::new(0.0, 710.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(MaterialPlugin::<CoolMaterial>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
            Quat::default(),
        ))
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CoolMaterial>>,
    mut pbr_materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((MaterialMeshBundle {
        mesh: mesh_assets.add(Mesh::from(Cylinder {
            radius: 0.75,
            height: 2.0,
            resolution: 32,
            segments: 4,
        })),
        material: materials.add(CoolMaterial {}),
        transform: Transform::from_xyz(-4.0, 0.0, 0.0),
        ..default()
    },
        Name::from("cylinder"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            sectors: 18,
            stacks: 9,
        })),
        material: pbr_materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
        transform: Transform::from_xyz(-4.0, 0.0, 0.0),
        ..default()
    },
        Name::from("sold sphere"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((MaterialMeshBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Box::new(2.0, 0.1, 2.0))),
        material: materials.add(CoolMaterial {}),
        transform: Transform::from_xyz(-2.0, 0.0, 0.0),
        ..default()
    },
        Name::from("plane"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((MaterialMeshBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Box::new(1.0, 2.0, 1.0))),
        material: materials.add(CoolMaterial {}),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
        Name::from("box"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((MaterialMeshBundle {
        mesh: mesh_assets.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(CoolMaterial { }),
        transform: Transform::from_xyz(2.0, 0.0, 0.0),
        ..default()
    },
        Name::from("uvsphere"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));
}

fn spawn_camera(mut commands: Commands) {
    let focus: Vec3 = Vec3::ZERO;
    let mut transform = Transform::default();
    transform.translation = Vec3 {
        x: -2.0,
        y: 2.5,
        z: 5.0,
    };
    transform.look_at(focus, Vec3::Y);

    let camera = Camera3dBundle {
        transform,
        ..Default::default()
    };

    commands.spawn((
        camera,
        PanOrbitCamera {
            radius: (transform.translation - focus).length(),
            focus,
            ..Default::default()
        },
        PickingCameraBundle::default(),
        bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}
