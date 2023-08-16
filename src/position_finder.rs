use crate::dice_gen::{DiceGen, FastrandDice};
use crate::position::GameState::Ongoing;
use crate::position::{Position, STARTING};
use std::collections::HashSet;

/// Finds random positions for later rollout.
pub struct PositionFinder {
    dice_gen: FastrandDice,
    rng: fastrand::Rng, // temporary, will be replaced with selection algorithm once first neural net is there.
}

impl PositionFinder {
    /// Contains different random number generators every time it's called.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_dice_gen(FastrandDice::new())
    }

    fn with_dice_gen(dice_gen: FastrandDice) -> Self {
        PositionFinder {
            dice_gen,
            rng: fastrand::Rng::new(),
        }
    }

    pub fn find_positions(&mut self, amount: usize) -> HashSet<Position> {
        let mut found: HashSet<Position> = HashSet::new();
        while found.len() < amount {
            let mut more = self.positions_in_one_random_game();
            while found.len() < amount {
                match more.pop() {
                    Some(pos) => found.insert(pos),
                    None => break,
                };
            }
        }
        found
    }

    fn positions_in_one_random_game(&mut self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        let mut pos = STARTING;
        while pos.game_state() == Ongoing {
            let (die1, die2) = self.dice_gen.roll();
            let new_positions = pos.all_positions_after_moving(die1, die2);
            let random_index = self.rng.usize(0..new_positions.len());
            // Todo: remove cloning by implementing the Copy trait -> maybe better performance
            pos = new_positions[random_index].clone();
            positions.push(pos.clone());
        }
        positions
    }
}