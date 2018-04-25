// Rework of https://github.com/ZeusWPI/MOZAIC/blob/00c51314fe226680dad21494c23b002e881c2829/gameserver/src/planetwars/pw_controller.rs

// Can't enable, newer version has incompatible interface.
// extern crate futures;

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

// use futures::{Async, Future, Poll};

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
    fn try_step(&mut self) -> Result<(), FailedStep> {
        // Validate conditions for state transitions
        let condition = true;
        if condition {
            // Replace the current machine with a new one.
            // This is allowed because every state of the machine has the SAME SIZE!
            take(self, |machine| {
                machine
            });

            return Ok(());
        }

        Err(FailedStep::WouldBlock)
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

/* Crashed functionalit */

impl PwController<Finished> {
    fn new() -> Self {
        unimplemented!()
    }
}

impl AsyncMachine for PwController<Finished> {
    fn try_step(&mut self) -> Result<(), FailedStep> {
        Err(FailedStep::Finished)
    }
}

/* Runtime dynamic behaviour */
#[derive(Debug)]
enum FailedStep {
    WouldBlock,
    Finished
    // TODO
}

trait AsyncMachine: Any + 'static {
    fn try_step(&mut self) -> Result<(), FailedStep>;
}

/* Futures integration */
struct PwControllerFuture {
    // TODO
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
            match self.controller.try_step() {
                Ok(_) => {},
                Err(e) => {
                    println!("{:?}", e);
                    return future::Poll((), ());
                }
            }
        }
    }
}

fn main() {
    let start = PwController::<Connecting>::new();
}
