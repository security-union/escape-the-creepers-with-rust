use crate::dijkstra::Dijkstra;
use gloo_console::log;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::Reducible;

pub type VertexId = (i32, i32);

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub row: i32,
    pub column: i32,
}

impl Location {
    pub fn id(&self) -> VertexId {
        (self.row, self.column)
    }
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
                let ferris = Ferris {
                    location: Location { row, column },
                    path: vec![],
                };
                let row = randy.gen_range(0..rows);
                let column = randy.gen_range(0..columns);
                let target = Location { row, column };
                let moves = vec![GameState { creepers, ferris }];

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

impl Game {
    pub fn get_adjacent_vertices(&self, vertex_id: VertexId) -> Vec<VertexId> {
        let (row, column) = vertex_id;
        let mut vertices: Vec<VertexId> = vec![];

        // left
        if column > 0 {
            // up
            if row > 0 {
                vertices.push((row - 1, column - 1));
            }
            // left
            vertices.push((row, column - 1));
            // bottom left
            if row < self.rows - 1 {
                vertices.push((row + 1, column - 1));
            }
        }
        // center
        {
            if row > 0 {
                vertices.push((row - 1, column));
            }
            // center bottom
            if row < self.rows - 1 {
                vertices.push((row + 1, column));
            }
        }
        // right
        if column < self.columns - 1 {
            // up
            if row > 0 {
                vertices.push((row - 1, column + 1));
            }
            // left
            vertices.push((row, column + 1));
            // bottom left
            if row < self.rows - 1 {
                vertices.push((row + 1, column + 1));
            }
        }
        vertices
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Location};

    #[test]
    fn get_adjacent_vertices_happy_path() {
        let game = Game {
            moves: vec![],
            rows: 10,
            columns: 10,
            target: Location { row: 0, column: 0 },
        };
        let adjacent_vertices = game.get_adjacent_vertices((5, 5));
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
            target: Location { row: 0, column: 0 },
        };
        let adjacent_vertices = game.get_adjacent_vertices((0, 0));
        let expected_vertices = vec![(1, 0), (0, 1), (1, 1)];
        assert_eq!(adjacent_vertices, expected_vertices);
    }

    #[test]
    fn get_adjacent_vertices_bottom_right() {
        let game = Game {
            moves: vec![],
            rows: 10,
            columns: 10,
            target: Location { row: 0, column: 0 },
        };
        let adjacent_vertices = game.get_adjacent_vertices((9, 9));
        let expected_vertices = vec![(8, 8), (9, 8), (8, 9)];
        assert_eq!(adjacent_vertices, expected_vertices);
    }
}
