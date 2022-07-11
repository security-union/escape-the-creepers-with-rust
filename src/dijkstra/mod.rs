use crate::model::{Game, Location};

/**
 *   A   →   2   →   B                  VERTEX     DISTANCE    LAST VERTEX
 *   |       ↑         ➘ 2                 A           0           A
 *   3       5          D                  B           2           A
 *   ↓       |         ➚ 4                 C           3           A
 *   C   →   6   →   E                     D           4           A
 *                                         E           9           A
 */

pub struct Dijkstra {}

impl Dijkstra {
    /**
     * Computes Dijkstra path from steve's current position to the target.
     */
    pub fn run(game_state: &Game) -> Vec<Location> {
        vec![]
    }
}
