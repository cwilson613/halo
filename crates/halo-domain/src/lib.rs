//! Bevy-independent authored document and deterministic simulation primitives.

/// Current native scenario schema version.
pub const SCENARIO_SCHEMA_VERSION: u32 = 1;

/// Stable authored identity. Runtime ECS entities are projections and never replace this value.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObjectId(u128);

impl ObjectId {
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u128 {
        self.0
    }
}

/// A minimal project-owned transform suitable for canonical documents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3 {
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform3 {
    fn default() -> Self {
        Self {
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioObject {
    pub id: ObjectId,
    pub name: String,
    pub transform: Transform3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioDocument {
    pub schema_version: u32,
    pub name: String,
    pub objects: Vec<ScenarioObject>,
}

impl ScenarioDocument {
    pub fn fixture() -> Self {
        Self {
            schema_version: SCENARIO_SCHEMA_VERSION,
            name: "phase-0-room".into(),
            objects: vec![ScenarioObject {
                id: ObjectId::from_u128(1),
                name: "fixture-cube".into(),
                transform: Transform3::default(),
            }],
        }
    }

    pub fn apply(&mut self, command: DomainCommand) -> Result<(), DomainError> {
        match command {
            DomainCommand::MoveObject { id, translation } => {
                let object = self
                    .objects
                    .iter_mut()
                    .find(|object| object.id == id)
                    .ok_or(DomainError::ObjectNotFound(id))?;
                object.transform.translation = translation;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DomainCommand {
    MoveObject { id: ObjectId, translation: [f32; 3] },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainError {
    ObjectNotFound(ObjectId),
}

/// Tiny deterministic state used to prove fixed-step traces without rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedStepBody {
    pub position: f64,
    pub velocity: f64,
}

impl FixedStepBody {
    pub fn step(&mut self, dt_seconds: f64) {
        self.position += self.velocity * dt_seconds;
    }
}

pub fn fixed_step_trace(mut body: FixedStepBody, dt_seconds: f64, steps: usize) -> Vec<f64> {
    let mut trace = Vec::with_capacity(steps);
    for _ in 0..steps {
        body.step(dt_seconds);
        trace.push(body.position);
    }
    trace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_command_moves_stable_object() {
        let mut document = ScenarioDocument::fixture();
        let id = document.objects[0].id;

        document
            .apply(DomainCommand::MoveObject {
                id,
                translation: [1.0, 2.0, 3.0],
            })
            .unwrap();

        assert_eq!(document.objects[0].id, id);
        assert_eq!(document.objects[0].transform.translation, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn fixed_step_trace_is_repeatable() {
        let body = FixedStepBody {
            position: 0.0,
            velocity: 3.0,
        };
        let first = fixed_step_trace(body, 1.0 / 60.0, 120);
        let second = fixed_step_trace(body, 1.0 / 60.0, 120);

        assert_eq!(first, second);
        assert_eq!(first.len(), 120);
        assert!((first[119] - 6.0).abs() < 1.0e-12);
    }

    #[test]
    fn unknown_object_is_an_error() {
        let mut document = ScenarioDocument::fixture();
        let missing = ObjectId::from_u128(404);
        assert_eq!(
            document.apply(DomainCommand::MoveObject {
                id: missing,
                translation: [0.0; 3],
            }),
            Err(DomainError::ObjectNotFound(missing))
        );
    }
}
