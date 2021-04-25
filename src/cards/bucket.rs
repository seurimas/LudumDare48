use super::DiggingCard;
use crate::prelude::*;
use log::info;

pub const BUCKET_SUCCESS_TIME: f32 = 1.;

#[derive(Debug, Clone, Copy)]
pub enum BucketAlertState {
    Empty,       // Don't do anything funky.
    Filled(f32), // Animate somehow!
}

#[derive(Debug)]
pub enum BucketState {
    Empty,
    Unheld(f32), // The player is not holding the mouse button down, and the bucket is .0 percent of the way to the top
    Held(f32), // The player is holding the mouse button down, and the bucket is held for n seconds
    Finished(f32), // The player has gotten to the top, and the bucket is now emptying. Used for animation, perhaps.
}

pub struct BucketUpdateSystem;

impl<'s> System<'s> for BucketUpdateSystem {
    type SystemData = (
        Write<'s, DiggingStatus>,
        WriteStorage<'s, DiggingCard>,
        Entities<'s>,
        Read<'s, Time>,
    );
    fn run(&mut self, (mut digging, mut cards, entities, time): Self::SystemData) {
        for (card, entity) in (&mut cards, &entities).join() {
            if let DiggingCard::Bucket(state) = card {
                match state {
                    BucketState::Held(progress) => {
                        *progress = *progress + time.delta_seconds();
                        if *progress > BUCKET_SUCCESS_TIME {
                            digging.empty_bucket();
                            if digging.no_buckets() {
                                info!("cleared buckets");
                                *card = DiggingCard::Bucket(BucketState::Finished(1.));
                            } else {
                                info!("cleared 1 bucket");
                                *card = DiggingCard::Bucket(BucketState::Unheld(*progress));
                            }
                        }
                    }
                    BucketState::Finished(_) => {
                        info!("destroy all buckets");
                        entities
                            .delete(entity)
                            .expect("Unreachable, entitity definitely exists");
                    }
                    _ => {}
                }
            }
        }
    }
}
