use crate::prelude::*;

#[derive(Debug)]
pub enum AlertState {
    Shovel,            // The shovel button is always the same
    BucketEmpty,       // Bucket is empty, just show an empty bucket button.
    BucketFilled(f32), // .0 is used to determine the color, so that we may pulse a color to indicate that a bucket needs to be emptied.
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Alertable {
    // An entity with this component will also have UI elements to display itself.
    // AlertableUpdateSystem will update the Alertable state based user interaction and timers.
    // AlertableRenderSystem will update the UI elements based on the Alertable state.
    pub state: AlertState,
    pub clicked: bool, // Whether it was clicked in the last frame.
}

#[derive(Debug)]
pub struct ShovelState {
    pub click_progress: f32, // How long has it been since we clicked? Used for animation, perhaps.
}

#[derive(Debug)]
pub enum BucketState {
    Unheld(f32), // The player is not holding the mouse button down, and the bucket is .0 percent of the way to the top
    Held(f32), // The player is holding the mouse button down, and the bucket is .0 percent of the way to the top
    Finished(f32), // The player has gotten to the top, and the bucket is now emptying. Used for animation, perhaps.
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum DiggingCard {
    // An entity with this component will also have multiple UI elements to display itself.
    // CardSpawningSystem will respond to clicks on Alertables and spawn a DiggingCard from a prefab.
    // CardUpdateSystem will update the internal state of these cards based on user input or whatever.
    // CardRenderingSystem will update the UI elements based on the internal state of these cards.
    Shovel(ShovelState),
    Bucket(BucketState),
}

pub struct AlertableUpdateSystem;

impl<'s> System<'s> for AlertableUpdateSystem {
    // Also needed: A resource which tracks the dirt state.
    // Also needed: Read UI input, likely like UiEventHandlerSystem: https://github.com/amethyst/amethyst/blob/main/examples/ui/main.rs
    type SystemData = (WriteStorage<'s, Alertable>, Read<'s, Time>);

    fn run(&mut self, (alertables, time): Self::SystemData) {
        /*
         Loop through alertables, update any timers, check if they have been clicked, fill buckets.
        */
        for alertable in alertables.join() {}
    }
}
pub struct AlertableRenderSystem;

impl<'s> System<'s> for AlertableRenderSystem {
    // Also needed: Components for UI, not sure what we'll use yet.
    type SystemData = (ReadStorage<'s, Alertable>,);

    fn run(&mut self, (alertables,): Self::SystemData) {
        /*
         Loop through alertables, update the UI based on the alertable state.
        */
        for alertable in alertables.join() {}
    }
}

pub struct CardSpawningSystem;

impl<'s> System<'s> for CardSpawningSystem {
    type SystemData = (ReadStorage<'s, Alertable>, Read<'s, LazyUpdate>);

    fn run(&mut self, (alertables, update): Self::SystemData) {
        /*
         Loop through alertables, check if any have been clicked based on the state. If so, spawn a card. Also, maybe, delete any old cards.
         It's maybe a good idea to split this up from AlertableSystem, just to keep systems dealing only with single responsibilities.
        */
        for alertable in alertables.join() {}
    }
}

pub struct CardUpdateSystem;

impl<'s> System<'s> for CardUpdateSystem {
    type SystemData = (
        WriteStorage<'s, DiggingCard>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (cards, input, update): Self::SystemData) {
        /*
         Loop through cards, check the mouse state, and update the card.
         We may need some sort of abstraction here (DiggingCard implements its own update function?) or just a simple match block.
        */
        for card in cards.join() {}
    }
}

pub struct CardRenderingSystem;

impl<'s> System<'s> for CardRenderingSystem {
    // I'm not 100% sure the component to use for the UI elements here. Probably UIContainer?
    type SystemData = (ReadStorage<'s, DiggingCard>,);

    fn run(&mut self, (cards,): Self::SystemData) {
        /*
         Loop through cards (really, only the one on screen, probably), update the UI based on card state.
        */
        for card in cards.join() {}
    }
}
