#![allow(dead_code, unused_variables)]
// Rework of https://github.com/ZeusWPI/MOZAIC/blob/00c51314fe226680dad21494c23b002e881c2829/gameserver/src/planetwars/pw_controller.rs

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

mod stub {
    pub mod slog {
        pub struct Logger;
    }

    pub mod future {
        pub struct Poll<X, Y>(pub X, pub Y);

        pub trait Future {
            type Item;
            type Error;

            fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
        }
    }

    pub struct PlayerLock;
    pub struct GameState;
    pub struct PlanetWars;
    pub struct PlayerId;
    pub struct ResponseValue;
}

mod marker {
    pub trait State {}
}

mod state {
    pub struct Connecting;
    pub struct Playing;
    pub struct Finished;

    use marker;
    impl marker::State for Connecting {}
    impl marker::State for Playing {}
    impl marker::State for Finished {}
}

use self::state::*;
use self::stub::*;

/* Runtime dynamic behaviour */
enum FailedStep {
    WouldBlock(Box<AsyncMachine>),
    Finished(Box<Finished>), // TODO
}

trait AsyncMachine: 'static {
    fn try_step(self: Box<Self>) -> Result<Box<AsyncMachine>, FailedStep>;
}

struct PwController<X: marker::State> {
    _state: PhantomData<X>,
    lock: PlayerLock,
    game_state: GameState,
    state: PlanetWars,
    planet_map: HashMap<String, usize>,
    logger: slog::Logger,
}

/* Connecting functionality */

impl PwController<Connecting> {
    fn new() -> Self {
        unimplemented!()
    }

    fn connect(
        self,
        messages: HashMap<PlayerId, ResponseValue>,
    ) -> Result<Self, PwController<Finished>> {
        unimplemented!()
    }
}

fn start_game(
    state: PwController<Connecting>,
) -> Result<PwController<Playing>, PwController<Connecting>> {
    unimplemented!()
}

impl AsyncMachine for PwController<Connecting> {
    fn try_step(self: Box<Self>) -> Result<Box<AsyncMachine>, FailedStep> {
        // Deref
        let me = *self;
        // Consumed state change
        let next_me: PwController<Playing> = me.into();
        // Return boxed version again
        let next_me: Box<AsyncMachine> = Box::new(next_me);
        return Ok(next_me);
    }
}

/* Playing functionality */

impl PwController<Playing> {
    fn step(
        self,
        messages: HashMap<PlayerId, ResponseValue>,
    ) -> Result<Self, PwController<Finished>> {
        unimplemented!()
    }
}

impl AsyncMachine for PwController<Playing> {
    fn try_step(self: Box<Self>) -> Result<Box<AsyncMachine>, FailedStep> {
        unimplemented!()
    }
}

/* Crashed functionalit */

impl PwController<Finished> {
    fn new() -> Self {
        unimplemented!()
    }
}

impl AsyncMachine for PwController<Finished> {
    fn try_step(self: Box<Self>) -> Result<Box<AsyncMachine>, FailedStep> {
        unimplemented!()
    }
}

/* Futures integration */
struct PwControllerFuture {
    pub controller: Box<(AsyncMachine + 'static)>,
}

// Functionality from take_mut, USE THAT CRATE!
// (Can't import because it's not part of the top-100 crates)
pub fn take<T, F>(mut_ref: &mut T, closure: F)
where
    F: FnOnce(T) -> T,
{
    use std::{panic, ptr};

    unsafe {
        let old_t = ptr::read(mut_ref);
        let new_t = panic::catch_unwind(panic::AssertUnwindSafe(|| closure(old_t)))
            .unwrap_or_else(|_| ::std::process::abort());
        ptr::write(mut_ref, new_t);
    }
}

impl future::Future for PwControllerFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> future::Poll<(), ()> {
        loop {
            take(&mut self.controller, |state| match state.try_step() {
                Ok(v) => v,
                Err(_) => panic!("error"),
            });
        }
    }
}

mod transition_impl {
    use super::*;

    impl From<PwController<Connecting>> for PwController<Playing> {
        fn from(old: PwController<Connecting>) -> Self {
            PwController {
                _state: PhantomData,
                lock: old.lock,
                game_state: old.game_state,
                state: old.state,
                planet_map: old.planet_map,
                logger: old.logger,
            }
        }
    }
}

fn main() {
    let start = PwController::<Connecting>::new();
}
