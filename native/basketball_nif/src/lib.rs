use rustler::types::atom::ok;
use rustler::{Encoder, Env, Error, NifUnitEnum, ResourceArc, Term};
use std::sync::RwLock;

#[derive(NifUnitEnum)]
pub enum Incident {
    OnePointer,
    TwoPointer,
    ThreePointer,
}

pub struct GameState {
    score: RwLock<u32>,
}

impl GameState {
    pub fn update(&self, incident: Incident) {
        use Incident::*;


        let increment = match incident {
            OnePointer => 1,
            TwoPointer => 2,
            ThreePointer => 3,
        };

        let mut score = self.score.write().unwrap();

        *score += increment;
    }

    pub fn get_score(&self) -> u32 {
        *self.score.read().unwrap()
    }
}

pub fn on_load<'a>(env: Env, _load_info: Term<'a>) -> bool {
    rustler::resource!(GameState, env);
    true
}

rustler::init! {
    "Elixir.BasketballNIF",
    [
        ("new", 0, new),
        ("update", 2, update),
        ("get_score", 1, get_score),
    ],
    Some(on_load)
}

fn new<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state = GameState {
        score: RwLock::new(0),
    };

    let resource = ResourceArc::new(state);

    Ok((ok(), resource).encode(env))
}

fn update<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state: ResourceArc<GameState> = args[0].decode()?;
    let incident: Incident = args[1].decode()?;

    state.update(incident);

    Ok((ok()).encode(env))
}

fn get_score<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state: ResourceArc<GameState> = args[0].decode()?;

    let score = state.get_score();

    Ok((ok(), score).encode(env))
}
