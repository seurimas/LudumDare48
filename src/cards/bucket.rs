use super::DiggingCard;
use crate::prelude::*;

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
        SoundPlayer<'s>,
    );
    fn run(&mut self, (mut digging, mut cards, entities, time, sounds): Self::SystemData) {
        for (card, entity) in (&mut cards, &entities).join() {
            if let DiggingCard::Bucket(state) = card {
                match state {
                    BucketState::Held(progress) => {
                        *progress = *progress + time.delta_seconds();
                        if *progress > BUCKET_SUCCESS_TIME {
                            digging.empty_bucket();
                            sounds.empty_bucket();
                            if digging.no_buckets() {
                                *card = DiggingCard::Bucket(BucketState::Finished(1.));
                            } else {
                                *card = DiggingCard::Bucket(BucketState::Unheld(*progress));
                            }
                        }
                    }
                    BucketState::Finished(_) => {
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

pub struct BucketRenderingSystem;

impl<'s> System<'s> for BucketRenderingSystem {
    // I'm not 100% sure the component to use for the UI elements here. Probably UIContainer?
    type SystemData = (
        Read<'s, DiggingStatus>,
        ReadStorage<'s, DiggingCard>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiImage>,
    );

    fn run(&mut self, (digging, cards, mut transforms, mut images): Self::SystemData) {
        /*
         Loop through cards (really, only the one on screen, probably), update the UI based on card state.
        */
        for card in cards.join() {
            match card {
                DiggingCard::Bucket(BucketState::Held(progress)) => {
                    for (mut transform, mut image) in (&mut transforms, &mut images).join() {
                        if transform.id.eq("dump_bucket_bar") {
                            let width = (1. - progress) * 117.;
                            let x = 65. + (progress * 117. * 0.5);
                            transform.local_x = x;
                            transform.width = width;
                        }
                    }
                }
                DiggingCard::Bucket(BucketState::Finished(_)) => {
                    for (mut transform, mut image) in (&mut transforms, &mut images).join() {
                        if transform.id.eq("dump_bucket_bar") {
                            let width = 0.;
                            let x = 65.;
                            transform.local_x = x;
                            transform.width = width;
                        }
                    }
                }
                DiggingCard::Bucket(BucketState::Unheld(_))
                | DiggingCard::Bucket(BucketState::Empty) => {
                    for (mut transform, mut image) in (&mut transforms, &mut images).join() {
                        if transform.id.eq("dump_bucket_bar") {
                            let width = 117.;
                            let x = 65.;
                            transform.local_x = x;
                            transform.width = width;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
