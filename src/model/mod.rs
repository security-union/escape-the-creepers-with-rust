use crate::dijkstra::Dijkstra;
use gloo_console::log;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::Reducible;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub row: i32,
    pub column: i32,
}

pub enum GameEvents {
    StartGameWithCreepers(i16, i32, i32),
    Tick, // Produced every time that we have to refresh.
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creeper {
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Steve {
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub creepers: Vec<Creeper>,
    pub steve: Steve,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub moves: Vec<GameState>,
    pub rows: i32,
    pub columns: i32,
    pub target: Location,
}

impl Reducible for Game {
    type Action = GameEvents;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        // process all events.
        match action {
            GameEvents::StartGameWithCreepers(creepers, rows, columns) => {
                // spawn creepers
                let mut randy = thread_rng();
                let creepers = (0..creepers)
                    .into_iter()
                    .map(|_i| {
                        let row = randy.gen_range(0..rows);
                        let column = randy.gen_range(0..columns);
                        Creeper {
                            location: Location { row, column },
                        }
                    })
                    .collect();
                // TODO: validate that steve does not spawn next or on top of a creeper.
                let row = randy.gen_range(0..rows);
                let column = randy.gen_range(0..columns);
                let steve = Steve {
                    location: Location { row, column },
                };
                let row = randy.gen_range(0..rows);
                let column = randy.gen_range(0..columns);
                let target = Location { row, column };
                let moves = vec![GameState { creepers, steve }];

                Game {
                    rows: rows,
                    columns: columns,
                    moves,
                    target,
                }
                .into()
            }
            GameEvents::Tick => {
                log!("tick");
                let result = Dijkstra::run(self.as_ref());
                self.clone().into()
            }
        }
    }
}
