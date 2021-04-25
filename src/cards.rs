use crate::prelude::*;
use log::info;

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
    Held(f32), // The player is holding the mouse button down, and the bucket is held for n seconds
    Finished(f32), // The player has gotten to the top, and the bucket is now emptying. Used for animation, perhaps.
}
const BUCKET_SUCCESS_TIME: f32 = 5.;

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
        Entities<'s>,
    );

    fn run(&mut self, (alertables, mut cards, mut spawner, entities): Self::SystemData) {
        /*
         Loop through alertables, check if any have been clicked based on the state. If so, spawn a card. Also, maybe, delete any old cards.
         It's maybe a good idea to split this up from AlertableSystem, just to keep systems dealing only with single responsibilities.
        */
        for alertable in alertables.join() {
            if alertable.clicked {
                for (_card, entity) in (&cards, &entities).join() {
                    entities.delete(entity).expect("Double delete");
                }
                let (prefab, card) = match alertable.state {
                    AlertState::Shovel => (
                        "prefabs/shovel_card.ron",
                        DiggingCard::Shovel(ShovelState { click_progress: 0. }),
                    ),
                    AlertState::Bucket(_) => (
                        "prefabs/bucket_card.ron",
                        DiggingCard::Bucket(BucketState::Empty),
                    ),
                };
                let entity = spawner.spawn_ui_widget(prefab, Position { x: 0., y: 0. });
                cards
                    .insert(entity, card)
                    .expect("Unreachable, entity just created");
            }
        }
    }
}

fn get_card_entity<'s>(
    entity: Entity,
    cards: &impl GenericReadStorage<Component = DiggingCard>,
    parents: &ReadStorage<'s, Parent>,
) -> Option<Entity> {
    if let Some(_card) = cards.get(entity) {
        Some(entity)
    } else if let Some(parent) = parents.get(entity) {
        get_card_entity(parent.entity, cards, parents)
    } else {
        None
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
        ReadStorage<'s, Parent>,
        ReadStorage<'s, UiTransform>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (events, mut digging, mut cards, input, parents, transforms, entities, time): Self::SystemData,
    ) {
        /*
         Loop through cards, check the mouse state, and update the card.
         We may need some sort of abstraction here (DiggingCard implements its own update function?) or just a simple match block.
        */
        for event in events.read(&mut self.reader_id) {
            if let Some((ent, mut card)) = get_card_entity(event.target, &cards, &parents)
                .and_then(|ent| cards.get_mut(ent).map(|card| (ent, card)))
            {
                match card {
                    DiggingCard::Shovel(_) => {
                        if event.event_type != UiEventType::Click {
                            continue;
                        }
                        if get_ui_name(event.target, &transforms).eq("shovel_dirt") {
                            digging.scoop();
                            if !digging.can_scoop() {
                                entities
                                    .delete(ent)
                                    .expect("Unreachable, entitity definitely exists");
                            }
                        }
                    }
                    DiggingCard::Bucket(bucket) => match bucket {
                        BucketState::Empty | BucketState::Unheld(_) => {
                            if event.event_type == UiEventType::ClickStart
                                && get_ui_name(event.target, &transforms).eq("fill_bucket")
                            {
                                *card = DiggingCard::Bucket(BucketState::Held(0.));
                            }
                        }
                        BucketState::Held(progress) => {
                            if event.event_type == UiEventType::HoverStop
                                || !get_ui_name(event.target, &transforms).eq("fill_bucket")
                            {
                                info!("unheld");
                                *card = DiggingCard::Bucket(BucketState::Unheld(*progress));
                            } else {
                                info!("held {}", progress);
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
                        }
                        BucketState::Finished(_) => {
                            entities
                                .delete(ent)
                                .expect("Unreachable, entitity definitely exists");
                        }
                    },
                }
            }
        }
    }
}

pub struct CardRenderingSystem;

impl<'s> System<'s> for CardRenderingSystem {
    // I'm not 100% sure the component to use for the UI elements here. Probably UIContainer?
    type SystemData = (
        Read<'s, DiggingStatus>,
        ReadStorage<'s, DiggingCard>,
        UiFinder<'s>,
        WriteStorage<'s, UiImage>,
    );

    fn run(&mut self, (digging, cards, finder, mut images): Self::SystemData) {
        /*
         Loop through cards (really, only the one on screen, probably), update the UI based on card state.
        */
        for card in cards.join() {
            match card {
                DiggingCard::Shovel(_) => {
                    if let Some(mut image) = finder
                        .find("shovel_bucket")
                        .and_then(|ent| images.get_mut(ent))
                    {
                        if digging.scoops_in_top_bucket() == 0 {
                            update_texture(image, Some(0.25), Some(0.375), Some(0.125), Some(0.25))
                        } else {
                            update_texture(
                                image,
                                Some(0.125 * digging.scoops_in_top_bucket() as f32 - 0.125),
                                Some(0.125 * digging.scoops_in_top_bucket() as f32),
                                Some(0.25),
                                Some(0.375),
                            )
                        }
                    }
                }
                _ => {}
            }
        }
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
