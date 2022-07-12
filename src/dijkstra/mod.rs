use std::{collections::HashMap, hash::Hash};

use priority_queue::DoublePriorityQueue;

use crate::model::{Game, Location, VertexId};

/**
 *   A   →   2   →   B                  VERTEX     DISTANCE    LAST VERTEX
 *   |       ↑         ➘ 2                 A           0           A
 *   3       5          D                  B           2           A
 *   ↓       |         ➚ 4                 C           3           A
 *   C   →   6   →   E                     D           4           A
 *                                         E           9           A
 */

#[derive(Debug)]
pub struct DistanceInfo {
    pub distance: Option<i32>,
    pub last_vertex: Option<VertexId>,
}

impl Default for DistanceInfo {
    fn default() -> Self {
        DistanceInfo {
            distance: None,
            last_vertex: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VertexInfo {
    pub vertex: VertexId,
    pub distance: i32,
}

#[derive(PartialEq)]
pub enum Mode {
    Creeper,
    Ferris,
}

pub struct Dijkstra {}

impl Dijkstra {
    /**
     * Computes Dijkstra path using mode
     */
    pub fn run(game: &Game, origin: &Location, target: &Location, mode: &Mode) -> Vec<Location> {
        let mut path = vec![];
        let distance_table = Dijkstra::build_distance_table(game, origin, target, mode);
        let mut stack = vec![];
        stack.push(target.clone());

        let mut previous_vertex = distance_table.get(&target.id()).unwrap().last_vertex;
        while let Some(unwrapped_vertex) = previous_vertex {
            if unwrapped_vertex == origin.id() {
                println!("we are done");
                break;
            }
            let (row, column) = unwrapped_vertex;
            stack.push(Location { x: row, y: column });
            previous_vertex = distance_table.get(&unwrapped_vertex).unwrap().last_vertex;
        }
        while let Some(location) = stack.pop() {
            path.push(location);
        }
        path
    }

    fn build_distance_table(
        game: &Game,
        origin: &Location,
        target: &Location,
        mode: &Mode,
    ) -> HashMap<VertexId, DistanceInfo> {
        // generate all nodes.
        let mut distance_table: HashMap<VertexId, DistanceInfo> = HashMap::new();
        let mut vertex_info_map: HashMap<VertexId, VertexInfo> = HashMap::new();
        let mut queue: DoublePriorityQueue<VertexId, i32> = DoublePriorityQueue::new();
        for row in 0..game.rows {
            for column in 0..game.columns {
                distance_table.insert(Location { x: row, y: column }.id(), DistanceInfo::default());
            }
        }
        let mut origin_distance_info = distance_table.get_mut(&origin.id()).unwrap();
        origin_distance_info.distance = Some(0);
        origin_distance_info.last_vertex = Some(origin.id());

        let source_vertex_info = VertexInfo {
            vertex: origin.id(),
            distance: 0,
        };
        vertex_info_map.insert(origin.id(), source_vertex_info.clone());
        queue.push(source_vertex_info.vertex, source_vertex_info.distance);

        while let Some(vertex_info) = queue.pop_min() {
            let current_vertex = vertex_info.0;
            for neighbor in game.get_adjacent_vertices(current_vertex, &target, mode) {
                // Get the new distance, account for the weighted edge.
                let distance = distance_table
                    .get(&neighbor)
                    .unwrap()
                    .distance
                    .map(|distance| {
                        distance + game.get_weighted_edge(current_vertex, neighbor, target, mode)
                    })
                    .unwrap_or(game.get_weighted_edge(current_vertex, neighbor, target, mode));

                // If we find a new shortest path to the neighbor update
                // the distance and the last vertex.
                let neighbor_vertex = distance_table.get_mut(&neighbor).unwrap();
                if neighbor_vertex.distance.is_none()
                    || neighbor_vertex.distance.unwrap() > distance
                {
                    let mut neighbor_vertex = distance_table.get_mut(&neighbor).unwrap();
                    neighbor_vertex.distance = Some(distance);
                    neighbor_vertex.last_vertex = Some(current_vertex);

                    if let Some(neighbor_vertex_info) = vertex_info_map.get(&neighbor) {
                        queue.remove(&neighbor_vertex_info.vertex);
                    }
                    let vertex_info = VertexInfo {
                        vertex: neighbor,
                        distance,
                    };
                    queue.push(vertex_info.vertex, vertex_info.distance);
                    vertex_info_map.insert(neighbor, vertex_info);
                }
            }
        }

        distance_table
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dijkstra::Mode,
        model::{Creeper, Game, GameState, Location, Status},
    };

    use super::Dijkstra;

    #[test]
    fn dijkstra_happy_path() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![],
                ferris: crate::model::Ferris {
                    location: Location { x: 0, y: 0 },
                    path: vec![],
                },
            }],
            rows: 4,
            columns: 4,
            target: Location { x: 0, y: 3 },
            status: Status::Idle,
        };
        let origin = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, origin, target, &Mode::Ferris);
        let expected_shortest_path = vec![
            Location { x: 0, y: 1 },
            Location { x: 0, y: 2 },
            Location { x: 0, y: 3 },
        ];
        assert_eq!(shortest_path, expected_shortest_path);
    }

    #[test]
    fn dijkstra_4_by_4() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![],
                ferris: crate::model::Ferris {
                    location: Location { x: 0, y: 0 },
                    path: vec![],
                },
            }],
            rows: 4,
            columns: 4,
            target: Location { x: 3, y: 3 },
            status: Status::Idle,
        };
        let origin = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, origin, target, &Mode::Ferris);
        let expected_shortest_path = vec![
            Location { x: 1, y: 1 },
            Location { x: 2, y: 2 },
            Location { x: 3, y: 3 },
        ];
        assert_eq!(shortest_path, expected_shortest_path);
    }

    #[test]
    fn dijkstra_8_by_8() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![],
                ferris: crate::model::Ferris {
                    location: Location { x: 2, y: 2 },
                    path: vec![],
                },
            }],
            rows: 8,
            columns: 8,
            target: Location { x: 7, y: 7 },
            status: Status::Idle,
        };
        let origin = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, origin, target, &Mode::Ferris);
        let expected_shortest_path = vec![
            Location { x: 3, y: 3 },
            Location { x: 4, y: 4 },
            Location { x: 5, y: 5 },
            Location { x: 6, y: 6 },
            Location { x: 7, y: 7 },
        ];
        assert_eq!(shortest_path, expected_shortest_path);
    }

    #[test]
    fn dijkstra_8_by_8_with_creeper() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![Creeper {
                    location: Location { x: 4, y: 4 },
                }],
                ferris: crate::model::Ferris {
                    location: Location { x: 2, y: 2 },
                    path: vec![],
                },
            }],
            rows: 8,
            columns: 8,
            target: Location { x: 7, y: 7 },
            status: Status::Idle,
        };
        let ferris_location = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, ferris_location, target, &Mode::Ferris);
        let expected_shortest_path = vec![
            Location { x: 3, y: 2 },
            Location { x: 4, y: 2 },
            Location { x: 5, y: 2 },
            Location { x: 6, y: 3 },
            Location { x: 7, y: 4 },
            Location { x: 7, y: 5 },
            Location { x: 7, y: 6 },
            Location { x: 7, y: 7 },
        ];

        assert_eq!(shortest_path, expected_shortest_path);
        let creeper_path = Dijkstra::run(
            &game,
            &game
                .moves
                .last()
                .unwrap()
                .creepers
                .first()
                .unwrap()
                .location,
            &ferris_location,
            &Mode::Creeper,
        );
        assert_eq!(
            creeper_path,
            vec![Location { x: 3, y: 3 }, Location { x: 2, y: 2 }]
        );
    }

    #[test]
    fn dijkstra_4_by_4_creeper_on_home() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![Creeper {
                    location: Location { x: 3, y: 3 },
                }],
                ferris: crate::model::Ferris {
                    location: Location { x: 0, y: 0 },
                    path: vec![],
                },
            }],
            rows: 4,
            columns: 4,
            target: Location { x: 3, y: 3 },
            status: Status::Idle,
        };
        let origin = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, origin, target, &Mode::Ferris);
        let expected_shortest_path = vec![
            Location { x: 1, y: 1 },
            Location { x: 2, y: 2 },
            Location { x: 3, y: 3 },
        ];
        assert_eq!(shortest_path, expected_shortest_path);
    }

    #[test]
    fn dijkstra_12_by_24() {
        let game = Game {
            moves: vec![GameState {
                creepers: vec![Creeper {
                    location: Location { x: 0, y: 9 },
                }],
                ferris: crate::model::Ferris {
                    location: Location { x: 4, y: 3 },
                    path: vec![],
                },
            }],
            rows: 24,
            columns: 12,
            target: Location { x: 5, y: 5 },
            status: Status::Idle,
        };
        let origin = &game.moves.last().unwrap().ferris.location;
        let target = &game.target;
        let shortest_path = Dijkstra::run(&game, origin, target, &Mode::Creeper);
        let expected_shortest_path = vec![Location { x: 5, y: 4 }, Location { x: 5, y: 5 }];
        assert_eq!(shortest_path, expected_shortest_path);
    }
}
