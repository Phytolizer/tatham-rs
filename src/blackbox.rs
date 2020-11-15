use rand::{Rng, SeedableRng};

#[derive(Debug, Eq, PartialEq)]
pub struct GameParams {
    width: usize,
    height: usize,
    min_balls: usize,
    max_balls: usize,
}

impl GameParams {
    pub fn new(
        width: usize,
        height: usize,
        min_balls: usize,
        max_balls: usize,
    ) -> GameParams {
        Self {
            width,
            height,
            min_balls,
            max_balls,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GameState {
    width: usize,
    height: usize,
    n_balls: usize,
    grid: Vec<Vec<bool>>,
}

pub fn new_game(game_params: GameParams) -> GameState {
    new_game_seeded(game_params, rand::thread_rng().gen())
}

pub fn new_game_seeded(game_params: GameParams, seed: u64) -> GameState {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let n_balls: usize =
        rng.gen_range(game_params.min_balls, game_params.max_balls + 1);
    let mut grid = Vec::with_capacity(game_params.width);
    grid.resize_with(game_params.width, || {
        let mut v = Vec::with_capacity(game_params.height);
        v.resize_with(game_params.height, || false);
        v
    });
    for _ in 0..n_balls {
        loop {
            let (x, y) = (
                rng.gen_range(0, game_params.width),
                rng.gen_range(0, game_params.height),
            );
            if grid[x][y] {
                continue;
            }
            grid[x][y] = true;
            break;
        }
    }
    GameState {
        width: game_params.width,
        height: game_params.height,
        n_balls,
        grid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_state() {
        assert_eq!(
            new_game_seeded(GameParams::new(3, 3, 1, 3), 1234),
            GameState {
                width: 3,
                height: 3,
                n_balls: 1,
                grid: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, true, false]
                ]
            }
        )
    }
}
