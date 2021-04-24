use crate::prelude::*;

#[derive(Debug)]
pub enum BucketAlertState {
    Empty,       // Don't do anything funky.
    Filled(f32), // Animate somehow!
}

#[derive(Debug)]
pub enum AlertState {
    Shovel,                   // The shovel button is always the same
    Bucket(BucketAlertState), // Bucket might animate
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
    Empty,
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

pub struct AlertableSpawningSystem;

impl<'s> System<'s> for AlertableSpawningSystem {
    type SystemData = (WriteStorage<'s, Alertable>, WidgetSpawner<'s>);

    fn run(&mut self, (alertables, spawner): Self::SystemData) {}
}

pub struct AlertableUpdateSystem {
    reader_id: ReaderId<UiEvent>,
}

impl<'s> System<'s> for AlertableUpdateSystem {
    // Also needed: A resource which tracks the dirt state.
    // Also needed: Read UI input, likely like UiEventHandlerSystem: https://github.com/amethyst/amethyst/blob/main/examples/ui/main.rs
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, Alertable>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self, (events, mut alertables, entities, time): Self::SystemData) {
        /*
         Loop through alertables, update any timers, check if they have been clicked, fill buckets.
        */
        for (mut alertable, entity) in (&mut alertables, &entities).join() {
            alertable.clicked = false;
        }
        for event in events.read(&mut self.reader_id) {
            if event.event_type != UiEventType::Click {
                continue;
            }
            if let Some(mut alertable) = alertables.get_mut(event.target) {
                alertable.clicked = true;
            }
        }
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
    type SystemData = (
        ReadStorage<'s, Alertable>,
        WriteStorage<'s, DiggingCard>,
        WidgetSpawner<'s>,
    );

    fn run(&mut self, (alertables, mut cards, mut spawner): Self::SystemData) {
        /*
         Loop through alertables, check if any have been clicked based on the state. If so, spawn a card. Also, maybe, delete any old cards.
         It's maybe a good idea to split this up from AlertableSystem, just to keep systems dealing only with single responsibilities.
        */
        for alertable in alertables.join() {
            if alertable.clicked {
                let entity = spawner.spawn_ui_widget("prefabs/card.ron", Position { x: 0., y: 0. });
                let card = match alertable.state {
                    AlertState::Shovel => DiggingCard::Shovel(ShovelState { click_progress: 0. }),
                    AlertState::Bucket(_) => DiggingCard::Bucket(BucketState::Empty),
                };
                cards
                    .insert(entity, card)
                    .expect("Unreachable, entity just created");
            }
        }
    }
}

pub struct CardUpdateSystem {
    reader_id: ReaderId<UiEvent>,
}

impl<'s> System<'s> for CardUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        Write<'s, DiggingStatus>,
        WriteStorage<'s, DiggingCard>,
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
    );

    fn run(&mut self, (events, mut digging, mut cards, input, entities): Self::SystemData) {
        /*
         Loop through cards, check the mouse state, and update the card.
         We may need some sort of abstraction here (DiggingCard implements its own update function?) or just a simple match block.
        */
        for event in events.read(&mut self.reader_id) {
            if event.event_type != UiEventType::Click {
                continue;
            }
            if let Some(mut card) = cards.get_mut(event.target) {
                match card {
                    DiggingCard::Shovel(_) => {
                        digging.scoop();
                        if !digging.can_scoop() {
                            entities
                                .delete(event.target)
                                .expect("Unreachable, entitity definitely exists");
                        }
                    }
                    DiggingCard::Bucket(_) => {
                        digging.empty_bucket();
                        if digging.no_buckets() {
                            entities
                                .delete(event.target)
                                .expect("Unreachable, entitity definitely exists");
                        }
                    }
                }
            }
        }
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

pub struct CardsBundle;

impl SystemBundle<'_, '_> for CardsBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        let mut ui_events = <Write<EventChannel<UiEvent>>>::fetch(world);
        dispatcher.add(AlertableSpawningSystem, "alert_spawn", &[]);
        let alert_reader = ui_events.register_reader();
        dispatcher.add(
            AlertableUpdateSystem {
                reader_id: alert_reader,
            },
            "alert_update",
            &[],
        );
        dispatcher.add(AlertableRenderSystem, "alert_render", &["alert_update"]);
        dispatcher.add(CardSpawningSystem, "card_spawn", &[]);
        let card_reader = ui_events.register_reader();
        dispatcher.add(
            CardUpdateSystem {
                reader_id: card_reader,
            },
            "card_update",
            &[],
        );
        dispatcher.add(CardRenderingSystem, "card_render", &["card_update"]);
        Ok(())
    }
}
