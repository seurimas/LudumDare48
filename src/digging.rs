use crate::prelude::*;

pub const SCOOPS_PER_BLOCK: u32 = 4;
pub const SCOOPS_PER_METER: u32 = 28;
pub const BLOCKS_PER_METER: u32 = SCOOPS_PER_METER / SCOOPS_PER_BLOCK;
pub const DRILL_METER: u32 = 1; // Raise before release.
pub const DRILL_TIME: f32 = 10.; // Raise before release.
pub const DRILL_SPEED: f32 = 4.; // Change before release.
pub const ROBOT_METER: u32 = 2; // Raise before release.
pub const ROBOT_TIME: f32 = 10.; // Raise before release.
pub const ROBOT_SPEED: f32 = 0.5; // Change before release.

#[derive(Clone, Copy)]
pub enum DrillStatus {
    Locked,
    Idling,
    Running { time_left: f32, partial_scoops: f32 },
}

#[derive(Clone, Copy)]
pub enum RobotStatus {
    Locked,
    Idling,
    Running {
        time_left: f32,
        partial_buckets: f32,
    },
}

pub struct DiggingStatus {
    scoops: u32,
    scoops_per_bucket: u32,
    pub time_since_shovel: f32,
    buckets: u32,
    depth: u32,
    progression: u32,
    progress_checks: u32,
    pub drill_status: DrillStatus,
    pub robot_status: RobotStatus,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Bucket {
    pub index: u32, // Which bucket are we.
}

impl Default for DiggingStatus {
    fn default() -> Self {
        DiggingStatus {
            scoops: 0,
            scoops_per_bucket: 8,
            time_since_shovel: 1.,
            buckets: 3,
            depth: 0,
            progression: 0,
            progress_checks: SCOOPS_PER_METER / 2,
            drill_status: DrillStatus::Locked,
            robot_status: RobotStatus::Locked,
        }
    }
}

impl DiggingStatus {
    pub fn scoop(&mut self, shovel: bool) {
        self.depth += 1;
        if shovel {
            self.time_since_shovel = 0.;
            self.scoops += 1;
        }
    }

    pub fn drill(&mut self) {
        self.drill_status = DrillStatus::Running {
            time_left: DRILL_TIME,
            partial_scoops: 0.,
        };
    }

    pub fn solve_captcha(&mut self) {
        self.robot_status = RobotStatus::Running {
            time_left: ROBOT_TIME,
            partial_buckets: 0.,
        };
    }

    pub fn empty_bucket(&mut self) {
        self.scoops = self.scoops - self.scoops_per_bucket;
    }

    pub fn scoops_in_top_bucket(&self) -> u32 {
        if self.scoops % self.scoops_per_bucket == 0 {
            if self.scoops == self.scoops_per_bucket * self.buckets {
                self.scoops_per_bucket
            } else {
                0
            }
        } else {
            self.scoops % self.scoops_per_bucket
        }
    }

    pub fn can_scoop(&self) -> bool {
        self.scoops < self.buckets * self.scoops_per_bucket
    }

    pub fn no_buckets(&self) -> bool {
        self.scoops < self.scoops_per_bucket
    }

    pub fn level(&self) -> u32 {
        self.depth / SCOOPS_PER_METER
    }

    pub fn block_index(&self) -> u32 {
        self.depth / SCOOPS_PER_BLOCK
    }

    pub fn current_block(&self) -> u32 {
        (self.depth % SCOOPS_PER_METER) / SCOOPS_PER_BLOCK
    }

    pub fn current_block_height(&self) -> u32 {
        (self.depth % SCOOPS_PER_METER) % SCOOPS_PER_BLOCK
    }

    pub fn get_depth_string(&self) -> String {
        format!("{:.3}", self.depth as f32 / SCOOPS_PER_METER as f32)
    }

    pub fn progress(&mut self) -> u32 {
        if self.depth >= (self.progression * self.progress_checks) + self.progress_checks {
            self.progression = self.depth / self.progress_checks;
            self.progression
        } else {
            0
        }
    }
}

pub struct DepthRenderSystem;

impl<'s> System<'s> for DepthRenderSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (
        Read<'s, DiggingStatus>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn run(&mut self, (digging, mut texts, finder): Self::SystemData) {
        if let Some(mut text) = finder
            .find("depth_indicator")
            .and_then(|ent| texts.get_mut(ent))
        {
            text.text = format!("Current Depth: {}", digging.get_depth_string());
        }
    }
}

pub struct BucketRenderSystem;

impl<'s> System<'s> for BucketRenderSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (
        Read<'s, DiggingStatus>,
        ReadStorage<'s, Bucket>,
        WriteStorage<'s, UiImage>,
        WriteStorage<'s, HiddenPropagate>,
        Entities<'s>,
    );

    fn run(&mut self, (digging, buckets, mut images, mut hidden, entities): Self::SystemData) {
        /*
         Loop through alertables, update the UI based on the alertable state.
        */
        for (bucket, mut image, entity) in (&buckets, &mut images, &entities).join() {
            if bucket.index >= digging.buckets && hidden.get(entity).is_none() {
                hidden
                    .insert(entity, HiddenPropagate::new())
                    .expect("Unreachable, definitely exists");
            } else if bucket.index < digging.buckets && hidden.get(entity).is_some() {
                hidden.remove(entity);
            }
            let filled_buckets = digging.scoops / digging.scoops_per_bucket;
            if let UiImage::PartialTexture { left, right, .. } = image {
                if bucket.index < filled_buckets {
                    *left = 0.125;
                    *right = 0.25;
                } else {
                    *left = 0.;
                    *right = 0.125;
                }
            }
        }
    }
}
pub struct ShovelTimingSystem;

impl<'s> System<'s> for ShovelTimingSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (Write<'s, DiggingStatus>, Read<'s, Time>);
    fn run(&mut self, (mut digging, time): Self::SystemData) {
        digging.time_since_shovel += time.delta_seconds();
    }
}

pub struct DrillDiggingSystem;

impl<'s> System<'s> for DrillDiggingSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (Write<'s, DiggingStatus>, Read<'s, Time>);
    fn run(&mut self, (mut digging, time): Self::SystemData) {
        let mut scooped = false;
        if digging.can_scoop() {
            if let DrillStatus::Running {
                time_left,
                partial_scoops,
            } = &mut digging.drill_status
            {
                *time_left -= time.delta_seconds();
                *partial_scoops += DRILL_SPEED * time.delta_seconds();
                if *partial_scoops > 1. {
                    *partial_scoops -= 1.;
                    scooped = true;
                }
                if *time_left < 0. {
                    digging.drill_status = DrillStatus::Idling;
                }
            }
        }
        if scooped {
            digging.scoop(false);
        }
    }
}

pub struct RobotRunningSystem;

impl<'s> System<'s> for RobotRunningSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (Write<'s, DiggingStatus>, Read<'s, Time>);
    fn run(&mut self, (mut digging, time): Self::SystemData) {
        let mut dumped = false;
        if !digging.no_buckets() {
            if let RobotStatus::Running {
                time_left,
                partial_buckets,
            } = &mut digging.robot_status
            {
                *time_left -= time.delta_seconds();
                *partial_buckets += ROBOT_SPEED * time.delta_seconds();
                if *partial_buckets > 1. {
                    *partial_buckets -= 1.;
                    dumped = true;
                }
                if *time_left < 0. {
                    digging.robot_status = RobotStatus::Idling;
                }
            }
        }
        if dumped {
            digging.empty_bucket();
        }
    }
}

pub struct ProgressionSystem;

impl<'s> System<'s> for ProgressionSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (
        Write<'s, DiggingStatus>,
        WriteStorage<'s, Alertable>,
        WidgetSpawner<'s>,
    );
    fn run(&mut self, (mut digging, mut alertables, mut spawner): Self::SystemData) {
        match digging.progress() {
            DRILL_METER => {
                digging.drill_status = DrillStatus::Idling;
                let alert_entity = spawner.spawn_ui_widget(
                    "prefabs/drill_alertable.ron",
                    Position { x: -64., y: -160. },
                );
                alertables
                    .insert(
                        alert_entity,
                        crate::cards::Alertable {
                            state: crate::cards::AlertState::Drill(
                                crate::cards::DrillAlertState::Ready,
                            ),
                            clicked: false,
                        },
                    )
                    .expect("Unreachable: entity just created");
            }
            ROBOT_METER => {
                digging.robot_status = RobotStatus::Idling;
                let alert_entity = spawner.spawn_ui_widget(
                    "prefabs/robot_alertable.ron",
                    Position { x: -64., y: -224. },
                );
                alertables
                    .insert(
                        alert_entity,
                        crate::cards::Alertable {
                            state: crate::cards::AlertState::Robot(
                                crate::cards::RobotAlertState::CaptchaNeeded,
                            ),
                            clicked: false,
                        },
                    )
                    .expect("Unreachable: entity just created");
            }
            _ => {}
        }
    }
}

pub struct DiggingBundle;

impl SystemBundle<'_, '_> for DiggingBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        world.insert(DiggingStatus::default());
        dispatcher.add(DepthRenderSystem, "depth_render", &[]);
        dispatcher.add(BucketRenderSystem, "bucket_render", &[]);
        dispatcher.add(ProgressionSystem, "progression", &[]);
        dispatcher.add(DrillDiggingSystem, "drill_digging", &[]);
        dispatcher.add(RobotRunningSystem, "robot_running", &[]);
        dispatcher.add(ShovelTimingSystem, "shovel_timing", &[]);
        Ok(())
    }
}
