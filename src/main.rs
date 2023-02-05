use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::settings::{WgpuFeatures, WgpuSettings},
    window::PresentMode, gltf::GltfMesh,
};

mod camera;
mod cylinder;
mod materials;

use camera::{CameraPlugin, PanOrbitCamera};
use cylinder::Cylinder;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use materials::{CoolMaterial, GeometryMaterial};

use crate::materials::JammyMaterial;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const WIDTH: f32 = 900.0;
//pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Resource)]
struct AppAssets {
    plane: Handle<GltfMesh>,
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
        .add_plugin(MaterialPlugin::<JammyMaterial>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
            Quat::default(),
        ))
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_to_stage(StartupStage::Startup, setup)
        .add_system(check_load)
        .run();
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let plane = assets.load("plane.gltf#Mesh0");
    commands.insert_resource(AppAssets { plane });
}

fn check_load(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    asset_server: Res<AssetServer>,
    mut loaded: Local<bool>,
    mut geo_materials: ResMut<Assets<GeometryMaterial>>,
    mut jammy_materials: ResMut<Assets<JammyMaterial>>,
    meshes: Res<Assets<GltfMesh>>,
) {
    use bevy::asset::LoadState;

    if !*loaded && asset_server.get_load_state(app_assets.plane.clone()) == LoadState::Loaded {
        let gltf_mesh = meshes.get(&app_assets.plane).unwrap();
        commands.spawn((
            MaterialMeshBundle {
                //mesh: mesh_assets.add(Mesh::from(shape::Box::new(2.0, 0.1, 2.0))),
                mesh: gltf_mesh.primitives[0].mesh.clone(),
                material: geo_materials.add(GeometryMaterial {}),
                transform: Transform::from_xyz(-2.0, 0.0, 0.0),
                ..default()
            },
            Name::from("plane"),
            PickableBundle::default(),
            bevy_transform_gizmo::GizmoTransformable,
            //Wireframe,
        ));

        commands.spawn((
            MaterialMeshBundle {
                //mesh: mesh_assets.add(Mesh::from(shape::Box::new(2.0, 0.1, 2.0))),
                mesh: gltf_mesh.primitives[0].mesh.clone(),
                material: jammy_materials.add(JammyMaterial {
                    color_texture: asset_server.load("images/map.png"),
                }),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            },
            Name::from("plane"),
            PickableBundle::default(),
            bevy_transform_gizmo::GizmoTransformable,
            //Wireframe,
        ));
        *loaded = true;
    }
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CoolMaterial>>,
    mut pbr_materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
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
        Name::from("sold sphere"),
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Box::new(1.0, 2.0, 1.0))),
            material: materials.add(CoolMaterial {}),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::from("box"),
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
