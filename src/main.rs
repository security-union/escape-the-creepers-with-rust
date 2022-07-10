use gloo_console::log;
use gloo_timers::callback::Interval;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VNode};

const ROWS: i32 = 24;
const COLUMNS: i32 = 12;
const CREEPERS: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    row: i32,
    column: i32,
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
}

impl Reducible for Game {
    type Action = GameEvents;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        // process all events.
        match action {
            GameEvents::StartGameWithCreepers(creepers, rows, columns) => {
                // spawn creepers
                log!("1");
                let mut randy = thread_rng();
                log!("2");
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
                log!("3");
                // TODO: validate that steve does not spawn next or on top of a creeper.
                let row = randy.gen_range(0..rows);
                log!("4");
                let column = randy.gen_range(0..columns);
                log!("5");
                let steve = Steve {
                    location: Location { row, column },
                };
                log!("6");
                let moves = vec![GameState { creepers, steve }];
                log!("7");

                Game {
                    rows: rows,
                    columns: columns,
                    moves,
                }
                .into()
            }
            GameEvents::Tick => {
                log!("tick");
                self.clone().into()
            }
        }
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct GameContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GameContextProvider)]
pub fn GameContextProviderImpl(props: &GameContextProviderProps) -> Html {
    let msg = use_reducer(|| Game {
        moves: vec![],
        rows: 0,
        columns: 0,
    });

    html! {
        <ContextProvider<UseReducerHandle<Game>> context={msg}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<Game>>>
    }
}

#[derive(PartialEq, Properties)]
struct CellProps {
    row: i32,
    column: i32,
}

#[function_component(Cell)]
fn cell(p: &CellProps) -> Html {
    let CellProps { row, column } = p;
    let game_state = use_context::<UseReducerHandle<Game>>().unwrap();
    // If creeper print it.
    let is_creeper = game_state
        .moves
        .last()
        .map({
            |game_move| {
                game_move
                    .creepers
                    .iter()
                    .find(|creeper| {
                        creeper.location.row == *row && creeper.location.column == *column
                    })
                    .is_some()
            }
        })
        .unwrap_or(false);

    html! {
        <div class = "cell">
            <div>{row}{","}{column}</div>
            <div>{is_creeper}</div>
        </div>
    }
}

#[function_component(GameRoot)]
fn game_root_component() -> Html {
    let game_state = use_context::<UseReducerHandle<Game>>().unwrap();

    use_effect_with_deps(
        move |_| {
            log!("rebuilding component");
            game_state.dispatch(GameEvents::StartGameWithCreepers(CREEPERS, ROWS, COLUMNS));
            let game_state = game_state.clone();

            // i intervals get out of scope they get dropped and destroyed
            let interval = Interval::new(100, move || game_state.dispatch(GameEvents::Tick));
            move || drop(interval)
        },
        (), // Only create the interval once per your component existence
    );

    fn column_generator(column: i32) -> Vec<VNode> {
        let rows: Vec<i32> = (0..ROWS).collect();
        rows.iter()
            .map(|row| {
                html! {
                    <Cell row={*row} column={column}/>
                }
            })
            .collect()
    }

    fn row_generator() -> Vec<VNode> {
        let rows: Vec<i32> = (0..COLUMNS).collect();
        rows.iter()
            .map(|j| {
                html! {
                    <>
                        {column_generator(*j)}
                    </>
                }
            })
            .collect()
    }

    html! {
        <div class="grid">
        {row_generator()}
    </div>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    html! {
        <GameContextProvider>
            <GameRoot/>
        </GameContextProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
