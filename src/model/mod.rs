use crate::dijkstra::{Dijkstra, Mode};
use gloo_console::log;
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    fmt::{self},
    rc::Rc,
};
use yew::Reducible;

pub type VertexId = (i32, i32);

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Location {
    pub fn id(&self) -> VertexId {
        (self.x, self.y)
    }

    pub fn from(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    pub fn move_direction(&self, direction: Direction, width: i32, height: i32) -> Location {
        match direction {
            Direction::Up => Some(Location {
                x: self.x,
                y: self.y - 1,
            })
            .filter(|_s| self.y > 0),
            Direction::Left => Some(Location {
                x: self.x - 1,
                y: self.y,
            })
            .filter(|_s| self.x > 0),
            Direction::Right => Some(Location {
                x: self.x + 1,
                y: self.y,
            })
            .filter(|_s| self.x < width - 1),
            Direction::Down => Some(Location {
                x: self.x,
                y: self.y + 1,
            })
            .filter(|_s| self.y < height - 1),
        }
        .unwrap_or(self.clone())
    }
}
pub enum GameEvents {
    InitGameWithCreepers(i16, i32, i32),
    Tick(i16), // Produced every time that we have to refresh.
    MoveFerris(Direction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Idle,
    Won,
    Lost,
    Playing,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Idle => write!(f, "Idle"),
            Status::Won => write!(f, "Won"),
            Status::Lost => write!(f, "Lost"),
            Status::Playing => write!(f, "Playing"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creeper {
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ferris {
    pub location: Location,
    pub path: Vec<Location>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub creepers: Vec<Creeper>,
    pub ferris: Ferris,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub moves: Vec<GameState>,
    pub rows: i32,
    pub columns: i32,
    pub target: Location,
    pub status: Status,
}

impl Reducible for Game {
    type Action = GameEvents;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        // process all events.
        match action {
            GameEvents::InitGameWithCreepers(creepers, rows, columns) => {
                // spawn creepers
                let mut randy = thread_rng();
                let creepers = (0..creepers)
                    .into_iter()
                    .map(|_i| {
                        let row = randy.gen_range(0..rows);
                        let column = randy.gen_range(0..columns);
                        Creeper {
                            location: Location { x: row, y: column },
                        }
                    })
                    .collect();
                // TODO: validate that steve does not spawn next or on top of a creeper.
                let row = randy.gen_range(0..rows);
                let column = randy.gen_range(0..columns);
                let ferris = Ferris {
                    location: Location { x: row, y: column },
                    path: vec![],
                };
                let row = randy.gen_range(0..rows);
                let column = randy.gen_range(0..columns);
                let target = Location { x: row, y: column };
                let moves = vec![GameState { creepers, ferris }];
                let mut game = Game {
                    rows: rows,
                    columns: columns,
                    moves,
                    target,
                    status: Status::Idle,
                };
                let origin = &game.moves.last().unwrap().ferris.location;
                let target = &game.target;
                let result = Dijkstra::run(&game, origin, &target, &Mode::Ferris);
                let mut ferris = &mut game.moves.last_mut().unwrap().ferris;
                ferris.path = result;
                game.validate_status();
                game.into()
            }
            GameEvents::Tick(tick) => {
                log!("tick {} {}", tick, self.moves.len() as u16);
                if self.status != Status::Playing {
                    return self.clone().into();
                }
                // On each tick, creepers have a chance to get closer to ferris,
                // Ferris has a chance to escape!!
                let game = self.clone();

                let mut moves = game.moves.clone();
                let mut last_move = game.moves.last().unwrap().clone();
                let ferris_location = &last_move.ferris.location;
                // move all creepers one block closer to ferris.
                if tick % 2 == 0 {
                    for mut creeper in last_move.creepers.iter_mut() {
                        let next_position = Dijkstra::run(
                            &game,
                            &creeper.location,
                            &ferris_location,
                            &Mode::Creeper,
                        );
                        if let Some(first) = next_position.first() {
                            creeper.location = first.clone();
                        }
                    }
                }

                moves.push(last_move.clone());
                let game = Game {
                    rows: game.rows,
                    columns: game.columns,
                    moves,
                    target: game.target.clone(),
                    status: game.status.clone(),
                };
                let mut mutable_game = game.clone();

                // move ferris
                let last_move = mutable_game.moves.last_mut().unwrap();
                let path = Dijkstra::run(&game, &ferris_location, &game.target, &Mode::Ferris);
                if let Some(first) = path.first() {
                    last_move.ferris.location = first.clone();
                }
                last_move.ferris.path = path;

                mutable_game.validate_status();
                mutable_game.into()
            }
            GameEvents::MoveFerris(direction) => {
                log!("status {}", self.status.to_string());
                if self.status != Status::Playing && self.status != Status::Idle {
                    return self.clone().into();
                }
                let game = self.clone();
                let mut status = game.status.clone();
                if status == Status::Idle {
                    status = Status::Playing;
                }

                let mut new_moves = self.moves.clone();
                let mut new_last_move = new_moves.last().unwrap().clone();
                let current_ferris_position = self.moves.last().unwrap().ferris.location.clone();
                new_last_move.ferris.location =
                    current_ferris_position.move_direction(direction, self.rows, self.columns);
                new_last_move.ferris.path = Dijkstra::run(
                    &game,
                    &new_last_move.ferris.location,
                    &game.target,
                    &Mode::Ferris,
                );
                if self.target == new_last_move.ferris.location {
                    status = Status::Won;
                }
                new_moves.push(new_last_move);

                let mut game = Game {
                    target: self.target.clone(),
                    rows: self.rows,
                    columns: self.columns,
                    moves: new_moves,
                    status,
                };
                game.validate_status();
                game.into()
            }
        }
    }
}

fn insert_adjacent_vertex(
    vector: &mut Vec<VertexId>,
    value: VertexId,
    creepers_map: &HashMap<VertexId, bool>,
    target: &Location,
    ferris_location: &Location,
) {
    let (row, column) = value;
    let location = Location::from(row, column);
    if !creepers_map.contains_key(&value) || location == *target || location == *ferris_location {
        vector.push(value);
    }
}

impl Game {
    pub fn get_adjacent_vertices(
        &self,
        vertex_id: VertexId,
        target: &Location,
        mode: &Mode,
    ) -> Vec<VertexId> {
        let (row, column) = vertex_id;
        let mut vertices: Vec<VertexId> = vec![];
        let mut creepers_map: HashMap<VertexId, bool> = HashMap::new();
        let ferris_location = self
            .moves
            .last()
            .map(|state| state.ferris.location.clone())
            .unwrap_or(Location { x: 0, y: 0 });
        if Mode::Creeper != *mode {
            if let Some(game_state) = self.moves.last() {
                for creeper in &game_state.creepers {
                    // do not insert just the creeper current location, add +1 -1 buffer around it.
                    let (x, y) = creeper.location.id();
                    creepers_map.insert((x, y), true);
                }
            }
        }
        // left
        if column > 0 {
            // up
            if row > 0 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row - 1, column - 1),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
            // left
            insert_adjacent_vertex(
                &mut vertices,
                (row, column - 1),
                &creepers_map,
                &target,
                &ferris_location,
            );
            // bottom left
            if row < self.rows - 1 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row + 1, column - 1),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
        }
        // center
        {
            if row > 0 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row - 1, column),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
            // center bottom
            if row < self.rows - 1 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row + 1, column),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
        }
        // right
        if column < self.columns - 1 {
            // up
            if row > 0 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row - 1, column + 1),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
            // left
            insert_adjacent_vertex(
                &mut vertices,
                (row, column + 1),
                &creepers_map,
                &target,
                &ferris_location,
            );

            // bottom left
            if row < self.rows - 1 {
                insert_adjacent_vertex(
                    &mut vertices,
                    (row + 1, column + 1),
                    &creepers_map,
                    &target,
                    &ferris_location,
                );
            }
        }
        vertices
    }

    pub fn get_weighted_edge(
        &self,
        _current_vertex: VertexId,
        neighbor: VertexId,
        target: &Location,
        mode: &Mode,
    ) -> i32 {
        let (row, column) = neighbor;
        let mut cost = (((target.x - row).pow(2) as f32 + (target.y - column).pow(2) as f32).sqrt()
            * 1000f32) as i32;
        if *mode == Mode::Ferris {
            let mut shortest_distance_to_creeper = f32::MAX;
            for creeper in &self.moves.last().unwrap().creepers {
                let creeper_location = &creeper.location;
                let distance = ((creeper_location.x - row).pow(2) as f32
                    + (creeper_location.y - column).pow(2) as f32)
                    .sqrt();
                if distance < shortest_distance_to_creeper {
                    shortest_distance_to_creeper = distance;
                }
            }
            if shortest_distance_to_creeper == 0f32 {
                shortest_distance_to_creeper = 1f32;
            }
            cost += (10000f32 / shortest_distance_to_creeper) as i32;
        }
        cost
    }

    pub fn validate_status(&mut self) {
        // If creeper hit ferris, user lost.
        if let Some(state) = self.moves.last() {
            if self.target == state.ferris.location {
                self.status = Status::Won;
                return;
            }
            let creepers = &state.creepers;
            for creeper in creepers {
                if creeper.location == state.ferris.location {
                    self.status = Status::Lost;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dijkstra::Mode,
        model::{Creeper, GameState, Status},
    };

    use super::{Game, Location};

    #[test]
    fn get_adjacent_vertices_happy_path() {
        let game = Game {
            moves: vec![],
            rows: 10,
            columns: 10,
            target: Location { x: 0, y: 0 },
            status: Status::Idle,
        };
        let adjacent_vertices = game.get_adjacent_vertices((5, 5), &game.target, &Mode::Ferris);
        let expected_vertices = vec![
            (4, 4),
            (5, 4),
            (6, 4),
            (4, 5),
            (6, 5),
            (4, 6),
            (5, 6),
            (6, 6),
        ];
        assert_eq!(adjacent_vertices, expected_vertices);
    }

    #[test]
    fn get_adjacent_vertices_top_left() {
        let game = Game {
            moves: vec![],
            rows: 10,
            columns: 10,
            target: Location { x: 0, y: 0 },
            status: Status::Idle,
        };
        let adjacent_vertices = game.get_adjacent_vertices((0, 0), &game.target, &Mode::Ferris);
        let expected_vertices = vec![(1, 0), (0, 1), (1, 1)];
        assert_eq!(adjacent_vertices, expected_vertices);
    }

    #[test]
    fn get_adjacent_vertices_bottom_right() {
        let game = Game {
            moves: vec![],
            rows: 10,
            columns: 10,
            target: Location { x: 0, y: 0 },
            status: Status::Idle,
        };
        let adjacent_vertices = game.get_adjacent_vertices((9, 9), &game.target, &Mode::Ferris);
        let expected_vertices = vec![(8, 8), (9, 8), (8, 9)];
        assert_eq!(adjacent_vertices, expected_vertices);
    }

    #[test]
    fn get_adjacent_vertices_with_creepers() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![Creeper {
                    location: Location { x: 5, y: 4 },
                }],
                ferris: crate::model::Ferris {
                    location: Location { x: 1, y: 1 },
                    path: vec![],
                },
            }],
            rows: 10,
            columns: 10,
            target: Location { x: 0, y: 0 },
            status: Status::Idle,
        };
        let adjacent_vertices = game.get_adjacent_vertices((5, 5), &game.target, &Mode::Ferris);
        let expected_vertices = vec![(4, 4), (6, 4), (4, 5), (6, 5), (4, 6), (5, 6), (6, 6)];
        assert_eq!(adjacent_vertices, expected_vertices);
    }
}
