use std::rc::Rc;

use gloo_timers::callback::Interval;
use survival::model::{Direction, Location, Status};
use survival::model::{Game, GameEvents};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use yew::{prelude::*, virtual_dom::VNode};

const ROWS: i32 = 24;
const COLUMNS: i32 = 12;
const CREEPERS: i16 = 10;

#[derive(Properties, Debug, PartialEq)]
pub struct GameContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}
#[derive(PartialEq, Properties)]
struct CellProps {
    row: i32,
    column: i32,
}


#[function_component(GameContextProvider)]
pub fn GameContextProviderImpl(props: &GameContextProviderProps) -> Html {
    let msg = use_reducer(|| Game {
        moves: vec![],
        rows: 0,
        columns: 0,
        target: Location { x: 0, y: 0 },
        status: Status::Idle,
    });


    html! {
        <ContextProvider<UseReducerHandle<Game>> context={msg}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<Game>>>
    }
}


#[function_component(Cell)]
fn cell(p: &CellProps) -> Html {
    let CellProps { row, column } = p;
    let current_location = Location {
        x: *row,
        y: *column,
    };
    let game_state = use_context::<UseReducerHandle<Game>>().unwrap();

    let is_creeper = game_state
        .moves
        .last()
        .map({
            |game_move| {
                game_move
                    .creepers
                    .iter()
                    .find(|creeper| creeper.location.x == *row && creeper.location.y == *column)
                    .is_some()
            }
        })
        .unwrap_or(false);

    let is_ferris = game_state
        .moves
        .last()
        .map(|g| g.ferris.location.x == *row && g.ferris.location.y == *column)
        .unwrap_or(false);

    let is_home = game_state.target.x == *row && game_state.target.y == *column;

    let is_path = game_state
        .moves
        .last()
        .map(|g| g.ferris.path.contains(&current_location))
        .unwrap_or(false);

    let ferris_image = if is_ferris {
        if is_home {
            html! {
                <img width="100%" src="thumbnail/win.png"/>
            }
        } else if is_creeper {
            html! {
                <img width="100%" src="thumbnail/lost.png"/>
            }
        } else {
            html! {
                <img width="100%" src="thumbnail/sadferris.png"/>
            }
        }
    } else {
        html! {
            <></>
        }
    };

    let creeper_image = if is_creeper && !is_ferris {
        html! {
            <img width="100%" src="thumbnail/creeper2.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    let home_image = if is_home && !is_ferris && !is_creeper {
        html! {
            <img width="100%" src="thumbnail/home.png"/>
        }
    } else {
        html! {
            <></>
        }
    };

    let is_path_image = if is_path && !is_home && !is_ferris {
        html! {
            <div class="blue_patch"/>
        }
    } else {
        html! {
            <></>
        }
    };

    html! {
        <div class = "cell">
            {creeper_image}
            {ferris_image}
            {home_image}
            {is_path_image}
        </div>
    }
}

#[function_component(GameRoot)]
fn game_root_component() -> Html {

    
    let game_state = Rc::new(use_context::<UseReducerHandle<Game>>().unwrap());
    let game_state_2 = game_state.clone();
    let game_state_3 = game_state.clone();
    use_effect_with_deps(
        move |_| {
            game_state.dispatch(GameEvents::InitGameWithCreepers(CREEPERS, ROWS, COLUMNS));
            let game_state = game_state.clone();
            let game_state_2 = game_state.clone();
            let mut counter = 0;

            

            let keyboard_callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let direction = match event.key().as_str() {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowLeft" => Some(Direction::Left),
                    "ArrowRight" => Some(Direction::Right),
                    "ArrowDown" => Some(Direction::Down),
                    _ => None,
                };
                if let Some(direction) = direction {
                    event.prevent_default();
                    game_state.dispatch(GameEvents::MoveFerris(direction));
                }
            }) as Box<dyn FnMut(_)>);
            let _result = window().unwrap().add_event_listener_with_callback(
                "keydown".into(),
                keyboard_callback.as_ref().unchecked_ref(),
            );
            keyboard_callback.forget();
            let interval = Interval::new(500, move || {
                counter += 1;
                game_state_2.dispatch(GameEvents::Tick(counter));
            });
            move || drop(interval)
        },
        (),
    );

    let restart_str = "Restart".to_string();
    let is_home = game_state_2.status == Status::Won;

    let instructions = match &(*game_state_2).status {
        Status::Idle => "Press any arrow key to start".to_string(),
        Status::Won => "Congrats, Ferris is home! please refresh to start another game".to_string(),
        Status::Lost => "We lost :( please refresh to start another game.".to_string(),
        Status::Playing => "Help Ferris to get home, avoid creepers. (if you do not press the arrows, Ferris will move on it's own)".to_string(),
        Status::Error(e) =>  format!("JEEEEZ, this is embarassing, but a bug creeped up {}", e.clone())
    };

    let handle_click_restart =  move |event:web_sys::MouseEvent| { 
        event.prevent_default();
        game_state_3.dispatch(GameEvents::InitGameWithCreepers(CREEPERS, ROWS, COLUMNS));
     };

    html! {
        <>
            { if is_home { html! {
                    <div class = "restart" type="restart">
                        <button class="restart_button" onclick={handle_click_restart}>{&restart_str}</button>
                    </div>
                } } else { html! { <></> } } 
            }
            <div class="status">
                <span class="center">{instructions}</span>
            </div>
            <div class="grid">
                {row_generator()}
            </div>
        </>
    }
}

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
