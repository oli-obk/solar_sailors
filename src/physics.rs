use std::ops::{Index, IndexMut};

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

impl Index<JointHandle> for Physics {
    type Output = Joint;

    #[track_caller]
    fn index(&self, handle: JointHandle) -> &Self::Output {
        self.joints.get(handle).unwrap()
    }
}

impl IndexMut<JointHandle> for Physics {
    #[track_caller]
    fn index_mut(&mut self, handle: JointHandle) -> &mut Self::Output {
        self.joints.get_mut(handle).unwrap()
    }
}

impl Index<RigidBodyHandle> for Physics {
    type Output = RigidBody;

    #[track_caller]
    fn index(&self, handle: RigidBodyHandle) -> &Self::Output {
        self.bodies.get(handle).unwrap()
    }
}

impl IndexMut<RigidBodyHandle> for Physics {
    #[track_caller]
    fn index_mut(&mut self, handle: RigidBodyHandle) -> &mut Self::Output {
        self.bodies.get_mut(handle).unwrap()
    }
}

impl Physics {
    pub fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -1.0],
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

    pub fn add_joint(
        &mut self,
        joint: impl Into<JointParams>,
        body1: RigidBodyHandle,
        body2: RigidBodyHandle,
    ) -> JointHandle {
        self.joints.insert(body1, body2, joint)
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
