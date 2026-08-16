use bevy::prelude::*;
use halo_domain::{DomainCommand, FixedStepBody, ObjectId, ScenarioDocument};

const FIXED_HZ: f64 = 60.0;

fn main() {
    let mut app = App::new();
    configure_domain_runtime(&mut app);
    app.insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.03)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Halo — Phase 0 Foundation".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_pbr_fixture.after(project_scenario));
    app.run();
}

#[derive(Resource)]
struct AuthoredScenario(ScenarioDocument);

#[derive(Resource, Default)]
struct PendingDomainCommands(Vec<DomainCommand>);

#[derive(Resource, Debug, PartialEq)]
struct SimulationState {
    body: FixedStepBody,
    steps: u64,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            body: FixedStepBody {
                position: 0.0,
                velocity: 3.0,
            },
            steps: 0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
struct AuthoredObjectId(ObjectId);

#[derive(Component)]
struct ScenarioProjection;

fn configure_domain_runtime(app: &mut App) {
    app.insert_resource(AuthoredScenario(ScenarioDocument::fixture()))
        .init_resource::<PendingDomainCommands>()
        .init_resource::<SimulationState>()
        .insert_resource(Time::<Fixed>::from_hz(FIXED_HZ))
        .add_systems(Startup, project_scenario)
        .add_systems(Update, (apply_domain_commands, sync_projection).chain())
        .add_systems(FixedUpdate, step_simulation);
}

fn project_scenario(mut commands: Commands, scenario: Res<AuthoredScenario>) {
    for object in &scenario.0.objects {
        commands.spawn((
            Name::new(object.name.clone()),
            AuthoredObjectId(object.id),
            ScenarioProjection,
            domain_transform(object.transform),
        ));
    }
}

fn apply_domain_commands(
    mut scenario: ResMut<AuthoredScenario>,
    mut pending: ResMut<PendingDomainCommands>,
) {
    for command in pending.0.drain(..) {
        // The command source owns user-facing diagnostics. Invalid commands cannot partially
        // mutate the document, so retaining the last valid projection is safe for this spike.
        let _ = scenario.0.apply(command);
    }
}

fn sync_projection(
    scenario: Res<AuthoredScenario>,
    mut projected: Query<(&AuthoredObjectId, &mut Transform), With<ScenarioProjection>>,
) {
    if !scenario.is_changed() {
        return;
    }

    for (id, mut transform) in &mut projected {
        if let Some(object) = scenario.0.objects.iter().find(|object| object.id == id.0) {
            *transform = domain_transform(object.transform);
        }
    }
}

fn domain_transform(transform: halo_domain::Transform3) -> Transform {
    Transform {
        translation: Vec3::from_array(transform.translation),
        rotation: Quat::from_array(transform.rotation_xyzw),
        scale: Vec3::from_array(transform.scale),
    }
}

fn step_simulation(mut simulation: ResMut<SimulationState>, fixed_time: Res<Time<Fixed>>) {
    simulation.body.step(fixed_time.delta_secs_f64());
    simulation.steps += 1;
}

fn setup_pbr_fixture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    projected: Query<Entity, With<ScenarioProjection>>,
) {
    let object_mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let object_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.34, 0.18),
        metallic: 0.35,
        perceptual_roughness: 0.55,
        ..default()
    });
    for entity in &projected {
        commands.entity(entity).insert((
            Mesh3d(object_mesh.clone()),
            MeshMaterial3d(object_material.clone()),
        ));
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    fn headless_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        configure_domain_runtime(&mut app);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / FIXED_HZ,
        )));
        app
    }

    #[test]
    fn domain_command_updates_authored_document_and_projection() {
        let mut app = headless_app();
        app.update();
        let id = ObjectId::from_u128(1);
        app.world_mut()
            .resource_mut::<PendingDomainCommands>()
            .0
            .push(DomainCommand::MoveObject {
                id,
                translation: [4.0, 5.0, 6.0],
            });

        app.update();

        assert_eq!(
            app.world().resource::<AuthoredScenario>().0.objects[0]
                .transform
                .translation,
            [4.0, 5.0, 6.0]
        );
        let mut query = app
            .world_mut()
            .query_filtered::<(&AuthoredObjectId, &Transform), With<ScenarioProjection>>();
        let (_, transform) = query.single(app.world()).unwrap();
        assert_eq!(transform.translation, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn fixed_schedule_produces_repeatable_sixty_hertz_trace() {
        fn run() -> SimulationState {
            let mut app = headless_app();
            app.update();
            for _ in 0..120 {
                app.update();
            }
            let state = app.world().resource::<SimulationState>();
            SimulationState {
                body: state.body,
                steps: state.steps,
            }
        }

        let first = run();
        let second = run();
        assert_eq!(first, second);
        assert_eq!(first.steps, 120);
        assert!((first.body.position - 6.0).abs() < 1.0e-6);
    }
}
