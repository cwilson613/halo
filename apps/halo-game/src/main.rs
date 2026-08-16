use bevy::prelude::*;
use halo_domain::ScenarioDocument;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.03)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Halo — Phase 0 Foundation".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let document = ScenarioDocument::fixture();
    let object = &document.objects[0];

    commands.spawn((
        Name::new(object.name.clone()),
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.34, 0.18),
            metallic: 0.35,
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_translation(Vec3::from_array(object.transform.translation)),
    ));

    commands.spawn((
        Name::new("ground"),
        Mesh3d(meshes.add(Cuboid::new(16.0, 0.25, 16.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.075, 0.09),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.125, 0.0),
    ));

    commands.spawn((
        Name::new("sun"),
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0)),
    ));

    commands.spawn((
        Name::new("phase-0-camera"),
        Camera3d::default(),
        Transform::from_xyz(6.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
