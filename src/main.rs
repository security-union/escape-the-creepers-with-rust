use gloo_console::log;
use survival::model::Location;
use survival::model::{Game, GameEvents};
use yew::{prelude::*, virtual_dom::VNode};

const ROWS: i32 = 24;
const COLUMNS: i32 = 12;
const CREEPERS: i16 = 10;

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
        target: Location { row: 0, column: 0 },
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
    let current_location = Location {
        row: *row,
        column: *column,
    };
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

    let is_ferris = game_state
        .moves
        .last()
        .map(|g| g.ferris.location.row == *row && g.ferris.location.column == *column)
        .unwrap_or(false);

    let is_home = game_state.target.row == *row && game_state.target.column == *column;

    let is_path = game_state
        .moves
        .last()
        .map(|g| g.ferris.path.contains(&current_location))
        .unwrap_or(false);

    let ferris_image = if is_ferris {
        html! {
            <img width="100%" src="thumbnail/sadferris.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    let creeper_image = if is_creeper {
        html! {
            <img width="100%" src="thumbnail/creeper2.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    let home_image = if is_home {
        html! {
            <img width="100%" src="thumbnail/home.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    let is_path_image = if is_path && !is_home {
        html! {
            <img width="100%" src="thumbnail/trail.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    html! {
        <div class = "cell">
            // <div>{row}{","}{column}</div>
            {creeper_image}
            {ferris_image}
            {home_image}
            {is_path_image}
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
            // let interval = Interval::new(100, move || game_state.dispatch(GameEvents::Tick));
            move || drop(game_state)
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
