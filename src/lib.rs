mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math; 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}
impl Cell {
	fn toggle(&mut self) {
		*self = match *self {
			Cell::Alive => Cell::Dead,
			Cell::Dead => Cell::Alive,
		}
	}
}

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Glider {
	NorthWest = 0,
	SouthWest = 1,
	NorthEast = 2,
	SouthEast = 3,
}

macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!( $( $t )* ).into())
	}
}

#[wasm_bindgen]
impl Universe {
	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn set_width(&mut self, w: u32) {
		self.width = w;
		self.cells = (0..w * self.height).map(|_i| Cell::Dead).collect();
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn set_height(&mut self, h: u32) {
		self.height = h;
		self.cells = (0..h * self.width).map(|_i| Cell::Dead).collect();
	}

	pub fn cells(&self) -> *const Cell {
		self.cells.as_slice().as_ptr()
	}

	pub fn tick(&mut self) {
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_ns = self.live_neighbor_count(row, col);

				next[idx] = match (cell, live_ns) {
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					(Cell::Dead, 3) => Cell::Alive,
					(otherwise, _) => otherwise,
				};
			}
		}
		self.cells = next;
	}

	pub fn create_glider(&mut self, row: u32, col: u32, g: Glider) {
		let w = self.width;
		let h = self.height;
		let cell_arr: [(u32, u32); 5] = match g {
			Glider::NorthWest => [(w-1,h-1), (w-1,0), (w-1,1), (0,h-1), (1,0)],
			Glider::SouthWest => [(w-1,1), (w-1,0), (w-1,h-1), (0,1), (1,0)],
			Glider::NorthEast => [(1,h-1), (1,0), (1,1), (0,h-1), (w-1,0)],
			Glider::SouthEast => [(1,1), (1,0), (1,h-1), (0,1), (w-1,0)],
		};
		for c in &cell_arr {
			let (delta_col, delta_row) = c;
			let n_row = (row + delta_row) % h;
			let n_col = (col + delta_col) % w;
			self.toggle_cell(n_row, n_col);
		}
	}

	pub fn create_pulsar(&mut self, row: u32, col: u32) {
		let w = self.width;
		let h = self.height;
		let cell_arr: [(u32, u32); 48] = [
			(w-6,h-2), (w-6,h-3), (w-6,h-4),
			(w-4,h-6), (w-3,h-6), (w-2,h-6),
			(w-1,h-2), (w-1,h-3), (w-1,h-4),
			(w-2,h-1), (w-3,h-1), (w-4,h-1),
			(6,h-2), (6,h-3), (6,h-4),
			(4,h-6), (3,h-6), (2,h-6),
			(1,h-2), (1,h-3), (1,h-4),
			(2,h-1), (3,h-1), (4,h-1),
			(6,2), (6,3), (6,4),
			(4,6), (3,6), (2,6),
			(1,2), (1,3), (1,4),
			(2,1), (3,1), (4,1),
			(w-6,2), (w-6,3), (w-6,4),
			(w-4,6), (w-3,6), (w-2,6),
			(w-1,2), (w-1,3), (w-1,4),
			(w-2,1), (w-3,1), (w-4,1),
		];
		for c in &cell_arr {
			let (delta_row, delta_col) = c;
			let n_row = (delta_row + row) % h;
			let n_col = (delta_col + col) % w;
			self.toggle_cell(n_row, n_col);
		}
	}

	fn get_index(&self, row: u32, col: u32) -> usize {
		(row * self.width + col) as usize
	}

	fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let n_row = (row + delta_row) % self.height;
				let n_col = (col + delta_col) % self.width;
				let idx = self.get_index(n_row, n_col);
				count += self.cells[idx] as u8;
			}
		}
		count
	}

	pub fn new(w: u32, h: u32) -> Self {
		utils::set_panic_hook();
		let rng = Math::random;
		let size = (w * h) as usize;
		let mut c = Vec::new();

		for _i in 0..size {
			c.push(if rng() > 0.75 {Cell::Alive} else {Cell::Dead});
		}

		Universe {
			width: w,
			height: h,
			cells: c,
		}
	}

	pub fn render(&self) -> String {
		self.to_string()
	}

	pub fn toggle_cell(&mut self, row: u32, col: u32) {
		let idx = self.get_index(row, col);
		self.cells[idx].toggle();
	}
}

impl Universe {
	pub fn get_cells(&self) -> &[Cell] {
		&self.cells
	}

	pub fn set_cells(&mut self, c: &[(u32, u32)]) {
		for (row, col) in c.iter().cloned() {
			let idx = self.get_index(row, col);
			self.cells[idx] = Cell::Alive;
		}
	}
}

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for r in self.cells.as_slice().chunks(self.width as usize) {
			for &c in r {
				let symbol = match c {
					Cell::Dead => "◻",
					Cell::Alive => "◼"
				};
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}