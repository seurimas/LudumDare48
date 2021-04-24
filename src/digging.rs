use crate::prelude::*;

pub struct DiggingStatus {
    scoops: u32,
    scoops_per_bucket: u32,
    buckets: u32,
    depth: u32,
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
            buckets: 3,
            depth: 0,
        }
    }
}

impl DiggingStatus {
    pub fn scoop(&mut self) {
        self.scoops = self.scoops + 1;
        self.depth = self.depth + 1;
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

pub struct DiggingBundle;

impl SystemBundle<'_, '_> for DiggingBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        world.insert(DiggingStatus::default());
        dispatcher.add(BucketRenderSystem, "bucket_render", &[]);
        Ok(())
    }
}
