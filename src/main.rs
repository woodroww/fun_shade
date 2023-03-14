use bevy::{
    gltf::GltfMesh,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::{AddressMode, SamplerDescriptor},
        settings::{WgpuFeatures, WgpuSettings},
        texture::ImageSampler, camera::ScalingMode,
    },
    window::PresentMode,
};

mod camera;
mod cylinder;
mod materials;
mod plane;

use camera::{CameraPlugin, PanOrbitCamera};
use cylinder::Cylinder;
use plane::SubdividedPlane;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use materials::{CoolMaterial, GeometryMaterial, HealthBarMaterial, WorldSpaceMaterial};

use crate::materials::{GLSLMaterial, MovingTextureMaterial};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const WIDTH: f32 = 900.0;
//pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Resource)]
struct AppAssets {
    gltf_plane: Handle<GltfMesh>,
    map_image: Handle<Image>,
    pattern_image: Handle<Image>,
    rock_image: Handle<Image>,
    health_image: Handle<Image>,
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
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugin(WireframePlugin)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(MaterialPlugin::<CoolMaterial>::default())
        .add_plugin(MaterialPlugin::<GeometryMaterial>::default())
        .add_plugin(MaterialPlugin::<GLSLMaterial>::default())
        .add_plugin(MaterialPlugin::<MovingTextureMaterial>::default())
        .add_plugin(MaterialPlugin::<WorldSpaceMaterial>::default())
        .add_plugin(MaterialPlugin::<HealthBarMaterial>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_to_stage(StartupStage::Startup, setup)
        .add_system(check_load)
        .add_system(cycle_health)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf_plane = asset_server.load("plane.gltf#Mesh0");
    let map_image = asset_server.load("images/map.png");
    let pattern_image = asset_server.load("images/pattern.png");
    let rock_image = asset_server.load("images/rock.png");
    let health_image = asset_server.load("images/healthbar.png");
    commands.insert_resource(AppAssets {
        gltf_plane,
        map_image,
        pattern_image,
        rock_image,
        health_image,
    });
}

fn check_load(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    asset_server: Res<AssetServer>,
    mut loaded: Local<bool>,
    mut glsl_materials: ResMut<Assets<GLSLMaterial>>,
    mut jam_materials: ResMut<Assets<MovingTextureMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut health_materials: ResMut<Assets<HealthBarMaterial>>,
) {
    use bevy::asset::LoadState;

    if !*loaded && asset_server.get_load_state(app_assets.map_image.clone()) == LoadState::Loaded {
        let plane_size = 1.0;
        match images.get_mut(&app_assets.map_image) {
            Some(mut image) => {
                let mut descriptor = SamplerDescriptor::default();
                descriptor.address_mode_u = AddressMode::Repeat;
                descriptor.address_mode_v = AddressMode::Repeat;
                image.sampler_descriptor = ImageSampler::Descriptor(descriptor);
            }
            None => {}
        }

        match images.get_mut(&app_assets.pattern_image) {
            Some(mut image) => {
                let mut descriptor = SamplerDescriptor::default();
                descriptor.address_mode_u = AddressMode::Repeat;
                descriptor.address_mode_v = AddressMode::Repeat;
                image.sampler_descriptor = ImageSampler::Descriptor(descriptor);
            }
            None => {}
        }

        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh_assets.add(Mesh::from(SubdividedPlane {
                    subdivisions: 2,
                    size: plane_size,
                })),
                material: jam_materials.add(MovingTextureMaterial {
                    color_texture: app_assets.map_image.clone(),
                    pattern: app_assets.pattern_image.clone(),
                    rock: app_assets.rock_image.clone(),
                }),
                transform: Transform::from_xyz(0.8, 0.0, 2.0),
                ..default()
            },
            Name::from("moving texture"),
            PickableBundle::default(),
            bevy_transform_gizmo::GizmoTransformable,
            Wireframe,
        ));

        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh_assets.add(Mesh::from(shape::Plane { size: plane_size })),
                material: glsl_materials.add(GLSLMaterial {
                    color_texture: app_assets.map_image.clone(),
                    color: Color::BLUE,
                    alpha_mode: AlphaMode::Blend,
                }),
                transform: Transform::from_xyz(2.0, 0.0, 2.0),
                ..default()
            },
            Name::from("plane glsl plane"),
            PickableBundle::default(),
            bevy_transform_gizmo::GizmoTransformable,
            Wireframe,
        ));

        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh_assets.add(Mesh::from(SubdividedPlane {
                    subdivisions: 0,
                    size: 1.0,
                })),
                material: health_materials.add(HealthBarMaterial {
                    health: 0.0,
                    color_texture: app_assets.health_image.clone(),
                }),
                transform: Transform::from_xyz(-2.0, 0.0, 2.0).with_scale(Vec3::new(1.0, 1.0, 0.125)),
                ..default()
            },
            Name::from("health"),
            PickableBundle::default(),
            bevy_transform_gizmo::GizmoTransformable,
            //Wireframe,
        ));

        *loaded = true;
    }
}

fn cycle_health(
    time: Res<Time>,
    mut health: ResMut<Assets<HealthBarMaterial>>,
) {
    //health.health = 0.5;
}

fn setup_segment_count(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut pbr_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 20,
                stacks: 8,
            })),
            material: pbr_materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            transform: Transform::from_xyz(-4.2, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(90.0f32.to_radians())),
            ..default()
        },
        Name::from("uvSphere2"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 8,
                stacks: 20,
            })),
            material: pbr_materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            transform: Transform::from_xyz(-2.1, 0.0, 0.0),
            ..default()
        },
        Name::from("uvSphere1"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Torus {
                radius: 0.7,
                ring_radius: 0.5,
                subdivisions_segments: 8,
                subdivisions_sides: 6,
            })),
            material: pbr_materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            transform: Transform::from_xyz(0.2, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(90.0f32.to_radians())),
            ..default()
        },
        Name::from("torus1"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));
    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Torus {
                radius: 0.7,
                ring_radius: 0.5,
                subdivisions_segments: 8,
                subdivisions_sides: 6,
            })),
            material: pbr_materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            transform: Transform::from_xyz(2.7, 0.0, 0.0),
            ..default()
        },
        Name::from("torus2"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));

}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CoolMaterial>>,
    mut pbr_materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut geo_materials: ResMut<Assets<GeometryMaterial>>,
    mut world_materials: ResMut<Assets<WorldSpaceMaterial>>,
) {
    wireframe_config.global = false;

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 4.0),
        ..default()
    });


    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh_assets.add(Mesh::from(SubdividedPlane {
                subdivisions: 25,
                size: 1.0,
            })),
            material: geo_materials.add(GeometryMaterial {}),
            transform: Transform::from_xyz(-2.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        Name::from("wavy plane"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        //Wireframe,
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(SubdividedPlane {
                subdivisions: 1,
                size: 1.0,
            })),
            material: pbr_materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, -2.0),
            ..default()
        },
        Name::from("simple divide"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Plane { size: 1.0 })),
            material: pbr_materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            transform: Transform::from_xyz(2.0, 0.0, -2.0),
            ..default()
        },
        Name::from("bevy plane"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
        Wireframe,
    ));

    commands.spawn((
        MaterialMeshBundle {
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

    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 18,
                stacks: 9,
            })),
            material: pbr_materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
            transform: Transform::from_xyz(-4.0, 0.0, 0.0),
            ..default()
        },
        Name::from("solid sphere"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: world_materials.add(WorldSpaceMaterial {}),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::from("uv box"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh_assets.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(CoolMaterial {}),
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
        x: 0.0,
        y: 0.0,
        z: 12.0,
    };
    //transform.look_at(focus, Vec3::Y);

    /*
    let camera = Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }),
        transform,
        ..default()
    };
    */

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
        Name::from("Main Camera"),
    ));
}
