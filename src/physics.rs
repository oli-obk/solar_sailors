use rapier2d::prelude::PhysicsPipeline;

use rapier2d::prelude::*;

pub struct Physics {
    pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -10.0],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn add(&mut self, rb: RigidBody) -> RigidBodyHandle {
        self.bodies.insert(rb)
    }

    pub fn get(&self, rb: RigidBodyHandle) -> &RigidBody {
        &self.bodies[rb]
    }

    pub fn update(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}
