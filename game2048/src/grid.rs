use rand::{seq::SliceRandom, distributions::Bernoulli, prelude::Distribution};

const MOVES: [Move; 4] = [Move::Left, Move::Right, Move::Up, Move::Down];
const CHANCE_FOR_TWO: f64 = 0.9;

#[derive(PartialEq, Debug)]
pub struct Grid {
    // 4x4 grid
    pub cells: [[u64; 4]; 4],
    score: u64
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Ok,
    InvalidMove,
    Lost,
}

impl Grid {

    fn new(cells: [[u64; 4]; 4]) -> Self {
        Self { cells, score: 0 }
    }

     pub fn new_random() -> Self {
        let cells = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let mut grid = Self::new(cells);
        grid.insert_random_cell();
        grid.insert_random_cell();
        grid
    }

    pub fn attempt(&mut self, mov: Move) -> GameStatus {
        if !self.move_is_valid(mov) {
            return GameStatus::InvalidMove;
        }

        let (new_cells, score_increase) = Self::make_move(self.cells, mov);
        self.cells = new_cells;
        self.score += score_increase;

        self.insert_random_cell();
        if self.has_player_lost() {
            return GameStatus::Lost;
        }
        GameStatus::Ok
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    fn insert_random_cell(&mut self) {
        if self.is_board_full() {
            return;
        }
        let mut rng = rand::thread_rng();
        let bern = Bernoulli::new(CHANCE_FOR_TWO).unwrap();
        let roll = bern.sample(&mut rng);
        let val: u64 = if roll { 2 } else { 4 };
        let empty_cells = Self::get_empty_cells(self.cells);
        // We know it can't be empty because we checked earlier so unwrapping is safe
        let (x, y) = empty_cells.choose(&mut rng).unwrap();
        self.cells[*x][*y] = val;
    }

    fn get_empty_cells(cells: [[u64; 4]; 4]) -> Vec<(usize, usize)> {
        let mut empty_cells: Vec<(usize, usize)> = Vec::new();

        for i in 0..4 {
            for j in 0..4 {
                if cells[i][j] == 0 {
                    empty_cells.push((i, j))
                }
            }
        }

        empty_cells
    }

    fn is_board_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|val| *val != 0))
    }

    fn move_is_valid(&self, mov: Move) -> bool {
        self.cells != (Self::make_move(self.cells, mov)).0
    }

    pub fn has_player_lost(&self) -> bool {
        !MOVES.iter().any(|mov| self.move_is_valid(*mov))
    }

    fn make_move(cells: [[u64; 4]; 4], mov: Move) -> ([[u64; 4]; 4], u64) {
        let rotation = mov.get_number();
        let (cells, score) = Self::handle_move(cells, rotation);
        (cells, score)
    }

    fn handle_move(cells: [[u64; 4]; 4], rotation: usize) -> ([[u64; 4]; 4], u64) {
        let rotated = Self::rotate_times(cells, rotation);
        let (cells, score) = Self::mov(rotated);
        let rotated_back = Self::rotate_times(cells, 4 - rotation);
        (rotated_back, score)
    }

    fn rotate_times(cells: [[u64; 4]; 4], n: usize) -> [[u64; 4]; 4] {
        let mut rotated_cells = cells;
        for _i in 0..n {
            rotated_cells = Self::rotate(rotated_cells);
        }
        rotated_cells
    }

    fn mov(cells: [[u64; 4]; 4]) -> ([[u64; 4]; 4], u64) {
        // Implementation of Going right.
        let mut cells = Self::mov_all_cells_to_the_side(cells);
        let mut score_increase: u64 = 0;

        for i in 0..4 {
            let old_row = cells[i];
            let mut new_row = old_row;

            for j in (1..=3).rev() {
                if new_row[j] == old_row[j - 1] {
                    new_row[j] *= 2;
                    score_increase += new_row[j];
                    new_row[j - 1] = 0;
                }
            }
            cells[i] = new_row;
        }
        (Self::mov_all_cells_to_the_side(cells), score_increase)
    }

    fn mov_all_cells_to_the_side(mut cells: [[u64; 4]; 4]) -> [[u64; 4]; 4] {
        for i in 0..4 {
            let mut row = cells[i];
            for j in (0..3).rev() {
                let temp = row[j];
                row[j] = 0;
                let mut index = j;
                while index < 3 {
                    if row[index + 1] != 0 {
                        break;
                    }
                    index += 1;
                }
                row[index] = temp;
            }
            cells[i] = row;
        }
        cells
    }

    fn rotate(mut cells: [[u64; 4]; 4]) -> [[u64; 4]; 4] {
        for i in 0..4 {
            for j in i..4 {
                let temp = cells[i][j];
                cells[i][j] = cells[j][i];
                cells[j][i] = temp;
            }
        }
        for i in 0..4 {
            cells[i].reverse()
        }
        cells
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new_random()
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Move {
    fn get_number(&self) -> usize {
        match self {
            Move::Right => 0,
            Move::Up => 1,
            Move::Left => 2,
            Move::Down => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, Move};

    #[test]
    fn get_empty_cells_work() {
        let row1 = [2, 2, 4, 2];
        let row2 = [2, 0, 2, 2];
        let row3 = [4, 2, 2, 0];
        let row4 = [2, 2, 2, 2];
        let grid = [row1, row2, row3, row4];
        let empty_cells = Grid::get_empty_cells(grid);

        assert_eq!(empty_cells, vec![(1, 1), (2, 3)])
    }

    #[test]
    fn move_right_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_right_works2() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_combination_works() {
        let row1 = [2, 2, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn cell_cant_combine_more_than_once() {
        let row1 = [2, 2, 4, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 8];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn doesnt_double_combinate_when_all_are_the_same() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn rotate_right_works() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 2, 0, 2];
        let row2 = [0, 0, 0, 2];
        let row3 = [0, 4, 0, 2];
        let row4 = [0, 0, 0, 2];
        let result_grid = [row1, row2, row3, row4];

        let grid = Grid::rotate(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn rotate_twice_works() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 0];
        let row2 = [0, 4, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [2, 2, 2, 2];
        let result_grid = [row1, row2, row3, row4];

        let grid = Grid::rotate_times(grid, 2);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn lol() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let grid = Grid::rotate_times(grid, 2);
        let grid = Grid::rotate_times(grid, 2);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check2() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 8];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 4];
        let row4 = [0, 0, 0, 2];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check3() {
        let row1 = [2, 2, 4, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check4() {
        let row1 = [2, 0, 2, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn something() {
        let row1 = [0, 0, 2, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 2, 2];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 8, 4];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_left_works() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [4, 8, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [4, 0, 0, 0];
        let row4 = [2, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::make_move(grid, Move::Left);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_up_works() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [4, 4, 4, 4];
        let row2 = [0, 0, 2, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::make_move(grid, Move::Up);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_down_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 2, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 4, 2];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::make_move(grid, Move::Down);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_all_to_the_side_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 2, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 2, 2];
        let row3 = [0, 0, 2, 2];
        let row4 = [0, 0, 2, 2];
        let result_grid = [row1, row2, row3, row4];

        let grid = Grid::mov_all_cells_to_the_side(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_all_to_the_side_works2() {
        let row1 = [2, 2, 4, 0];
        let row2 = [2, 2, 0, 2];
        let row3 = [4, 0, 2, 2];
        let row4 = [0, 2, 2, 2];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 2, 2, 4];
        let row2 = [0, 2, 2, 2];
        let row3 = [0, 4, 2, 2];
        let row4 = [0, 2, 2, 2];
        let result_grid = [row1, row2, row3, row4];

        let grid = Grid::mov_all_cells_to_the_side(grid);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn random_cell_is_inserted_after_attempt() {
        let row1 = [0, 0, 2, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 8, 4];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.attempt(Move::Right);
        assert_ne!(grid, result_grid);
    }

    #[test]
    fn board_is_full_works() {
        let row1 = [2, 2, 4, 2];
        let row2 = [2, 2, 2, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert_eq!(grid.is_board_full(), true);

        let row1 = [2, 2, 4, 2];
        let row2 = [2, 2, 0, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert_eq!(grid.is_board_full(), false)
    }

    #[test]
    fn insert_random_cell_works() {
        let row1 = [2, 2, 4, 2];
        let row2 = [2, 0, 2, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);
        grid.insert_random_cell();

        let cell = grid.cells[1][1];
        assert!(cell == 2 || cell == 4)
    }

    #[test]
    fn nothing_moves_when_nothing_should_move() {
        let row1 = [2, 4, 8, 16];
        let row2 = [2, 4, 8, 16];
        let row3 = [2, 4, 8, 16];
        let row4 = [2, 4, 8, 16];
        let grid = [row1, row2, row3, row4];

        let row1 = [2, 4, 8, 16];
        let row2 = [2, 4, 8, 16];
        let row3 = [2, 4, 8, 16];
        let row4 = [2, 4, 8, 16];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);

        assert_eq!(grid, result_grid)
    }

    #[test]
    fn bad_feeling_bout_this() {
        let row1 = [2, 2, 2, 4];
        let row2 = [2, 4, 4, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let row1 = [0, 2, 4, 4];
        let row2 = [0, 2, 4, 8];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = [row1, row2, row3, row4];

        let (grid, _) = Grid::mov(grid);
        assert_eq!(grid, result_grid);
    }

    #[test]
    fn has_player_lost_works_when_player_board_isnt_full() {
        let row1 = [2, 2, 2, 4];
        let row2 = [2, 4, 4, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert!(!grid.has_player_lost())
    }

    #[test]
    fn has_player_lost_works_when_player_board_is_full_but_a_move_is_possible() {
        let row1 = [2, 4, 2, 4];
        let row2 = [4, 2, 4, 2];
        let row3 = [2, 4, 2, 4];
        let row4 = [4, 2, 4, 4];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert!(!grid.has_player_lost())
    }

    #[test]
    fn player_has_lost_when_player_has_lost() {
        let row1 = [2, 4, 2, 4];
        let row2 = [4, 2, 4, 2];
        let row3 = [2, 4, 2, 4];
        let row4 = [4, 2, 4, 2];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert!(grid.has_player_lost())
    }

    #[test]
    fn move_is_valid_works_when_invalid() {
        let row1 = [2, 4, 0, 0];
        let row2 = [2, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 2, 0, 0];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert!(!grid.move_is_valid(Move::Left))
    }

    #[test]
    fn move_is_valid_works_when_valid() {
        let row1 = [2, 4, 0, 0];
        let row2 = [2, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 2, 0, 0];
        let grid = Grid::new([row1, row2, row3, row4]);

        assert!(grid.move_is_valid(Move::Right))
    }

    #[test]
    fn grid_has_2_random_cells_after_being_created() {
        let grid = Grid::new_random();
        let empty_cells = Grid::get_empty_cells(grid.cells);
        assert_eq!(empty_cells.len(), 16-2);
    }

    #[test]
    fn score_increase_should_be_16_plus_8() {
        let row1 = [4, 4, 0, 0];
        let row2 = [2, 4, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 8, 8, 0];
        let grid = [row1, row2, row3, row4];
        let (_, score) = Grid::mov(grid);

        assert_eq!(score, 24)
    }

    #[test]
    fn get_score_works() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let grid = [row1, row2, row3, row4];

        let mut grid = Grid::new(grid);
        grid.attempt(Move::Right);
        grid.attempt(Move::Right);

        assert_eq!(grid.get_score(), 4+4+8)
    }
}
