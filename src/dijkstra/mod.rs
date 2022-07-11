use std::{collections::HashMap, hash::Hash};

use priority_queue::PriorityQueue;

use crate::model::{Game, Location, VertexId};

/**
 *   A   →   2   →   B                  VERTEX     DISTANCE    LAST VERTEX
 *   |       ↑         ➘ 2                 A           0           A
 *   3       5          D                  B           2           A
 *   ↓       |         ➚ 4                 C           3           A
 *   C   →   6   →   E                     D           4           A
 *                                         E           9           A
 */

pub struct DistanceInfo {
    pub distance: f32,
    pub last_vertex: VertexId,
}

impl Default for DistanceInfo {
    fn default() -> Self {
        DistanceInfo {
            distance: 0f32,
            last_vertex: (0i32, 0i32),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VertexInfo {
    pub vertex: VertexId,
    pub distance: i32,
}

// impl PartialEq for VertexInfo {
//     fn eq(&self, other: &Self) -> bool {
//         self.vertex == other.vertex
//     }
// }

pub struct Dijkstra {}

impl Dijkstra {
    /**
     * Computes Dijkstra path from steve's current position to the target.
     */
    pub fn run(game: &Game) -> Vec<Location> {
        let table = Dijkstra::build_distance_table(game);
        vec![]
    }

    fn build_distance_table(game: &Game) -> HashMap<VertexId, DistanceInfo> {
        // Create adjacency table
        // start location = ferris location.
        let last_state = game.moves.last().unwrap();
        let ferris_location = &last_state.ferris.location;
        let target = &game.target;

        // generate all nodes.
        let mut distance_table: HashMap<VertexId, DistanceInfo> = HashMap::new();
        let mut vertex_info_map: HashMap<VertexId, VertexInfo> = HashMap::new();
        let mut queue: PriorityQueue<VertexId, i32> = PriorityQueue::new();
        for row in 1..game.rows {
            for column in 1..game.columns {
                distance_table.insert(Location { row, column }.id(), DistanceInfo::default());
            }
        }
        let mut ferris_distance_info = distance_table.get_mut(&ferris_location.id()).unwrap();
        ferris_distance_info.distance = 0.0;
        ferris_distance_info.last_vertex = ferris_location.id();

        let source_vertex_info = VertexInfo {
            vertex: ferris_location.id(),
            distance: 0,
        };
        vertex_info_map.insert(ferris_location.id(), source_vertex_info.clone());
        queue.push(source_vertex_info.vertex, source_vertex_info.distance);

        while let Some(vertex_info) = queue.pop() {
            let current_vertex = vertex_info.0;
            for neighbor in game.get_adjacent_vertices(current_vertex) {}
        }

        distance_table
    }
}
