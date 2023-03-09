use rand::Rng;

use crate::constants::TileType;

// TODO: This might be a good candidate for a command buffer.
pub struct Tilemap {
    tiles: Vec<Vec<TileType>>,
}

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Tilemap {
        Tilemap {
            tiles: vec![vec![TileType::Ground; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &TileType {
        &self.tiles[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[y][x] = tile;
    }

    pub fn apply_rect_to_map(&mut self, rect: &URect, tile: TileType) {
        for x in rect.x..rect.x + rect.width {
            for y in rect.y..rect.y + rect.height {
                self.set_tile(x, y, tile);
            }
        }
    }

    pub fn apply_rect_border_to_map(&mut self, rect: &URect, tile: TileType) {
        for x in rect.x..rect.x + rect.width {
            self.set_tile(x, rect.y, tile);
            self.set_tile(x, rect.y + rect.height - 1, tile);
        }

        for y in rect.y..rect.y + rect.height {
            self.set_tile(rect.x, y, tile);
            self.set_tile(rect.x + rect.width - 1, y, tile);
        }
    }

    pub fn apply_border_to_map(&mut self, tile: TileType) {
        for x in 0..self.tiles[0].len() {
            self.set_tile(x, 0, tile);
            self.set_tile(x, self.tiles.len() - 1, tile);
        }

        for y in 0..self.tiles.len() {
            self.set_tile(0, y, tile);
            self.set_tile(self.tiles[0].len() - 1, y, tile);
        }
    }

    /// Applies a line of a given tile type to the map between the specified start and end points
    pub fn apply_line_to_map(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        tile: TileType,
    ) {
        // initialize the current position to the starting position
        let (mut current_row, mut current_col) = start;
        let (end_row, end_col) = end;

        // calculate the absolute difference between the starting and ending positions
        let delta_row = (end_row as i32 - current_row as i32).abs();
        let delta_col = (end_col as i32 - current_col as i32).abs();

        // determine the direction to step in both dimensions. For example
        // if the starting position is (0, 0) and the ending position is (2, 2),
        // the step in the row dimension will be 1 and the step in the column
        // dimension will be 1. If the starting position is (2, 2) and the
        // ending position is (0, 0), the step in the row dimension will be -1
        // and the step in the column dimension will be -1.
        let step_row = if current_row < end_row { 1 } else { -1 };
        let step_col = if current_col < end_col { 1 } else { -1 };

        // initialize the error term to the negative column difference
        let mut error = delta_row - delta_col;

        loop {
            // set the tile at the current position to the specified tile type
            self.set_tile(current_row, current_col, tile);

            if current_row == end_row && current_col == end_col {
                // if we've reached the end point, break out of the loop
                break;
            }

            // calculate the error term multiplied by 2. We multiply by 2
            // because we're going to add it to the error term in the column
            // dimension and subtract it from the error term in the row
            // dimension. Multiplying by 2 allows us to avoid a division
            // operation.
            let double_error = 2 * error;

            if double_error > -delta_col {
                // if the error term in the column dimension is greater than the
                // negative column difference, update the row position and adjust
                // the error term.
                error -= delta_col;
                current_row = (current_row as i32 + step_row) as usize;
            }

            if double_error < delta_row {
                // if the error term in the row dimension is less than the row
                // difference, update the column position and adjust the error term
                error += delta_row;
                current_col = (current_col as i32 + step_col) as usize;
            }
        }
    }
}

pub struct URect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl URect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> URect {
        URect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> (usize, usize) {
        let center_x = self.x + self.width / 2;
        let center_y = self.y + self.height / 2;
        (center_x, center_y)
    }

    pub fn intersect(&self, other: &URect) -> bool {
        (self.x <= other.x + other.width)
            && (self.x + self.width >= other.x)
            && (self.y <= other.y + other.height)
            && (self.y + self.height >= other.y)
    }
}
