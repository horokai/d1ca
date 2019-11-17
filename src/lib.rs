//
// D1caUniverse
//

mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct D1caUniverse {
    width: u32,
    order: u32,
    cells: Vec<u8>,

    direction: u8,

    // Square lattice to show the time evolution of cells
    lattice: Vec<u8>,
}

#[wasm_bindgen]
impl D1caUniverse {
    // cells の更新を行う
    pub fn tick(&mut self) {
        // 規則番号からアルゴリズムを取得する
        let mut rule = Vec::new();
        let mut buff = self.order;
        for _ in 0..6 {
            rule.push((buff % 2) as u8);
            buff = buff / 2;
        }
        // 現在情報を保存しておく
        // Note : clone_from_slice は 長さが同じスライスでないと機能しない
        let mut old = vec![0; self.width as usize];
        old.clone_from_slice(&self.cells);

        // for loop による処理
        for i in 0..self.width {
            let mut count: u8 = 0;
            for j in 0..5 {
                count += old[((self.width + i + j - 1 - self.direction as u32) % self.width) as usize];
            }
            self.cells[i as usize] = rule[count as usize]; // 配列のインデックスは usize でないと排除
        }
    }

    // 正方格子として時間発展を追跡するためのメソッド : wasm でブラウザ表示するためには、rust 上で処理するのが一番よろしい
    pub fn tick_lattice(&mut self) {
        // next_line を取得
        self.tick();
        // 下方に Lattice をズラす
        let width = self.width;
        let max_idx = width * width - 1;
        for idx in 0..width * (width - 1) {
            self.lattice[(max_idx - idx) as usize] = self.lattice[(max_idx - idx - width) as usize];
        }
        // 先頭行を next_line の値とする
        for idx in 0..width {
            self.lattice[idx as usize] = self.cells[idx as usize];
        }
    }

    // width を引数に取る
    // 初期状態は乱数で決定する
    // seed はランダム。技術的な問題
    pub fn new(width: u32, order: u32) -> D1caUniverse {
        let cells = (0..width)
            .map(|_| if rand::random::<u32>() % 2 == 0 { 0 } else { 1 })
            .collect();

        // 正方格子の情報
        let lattice = vec![0 as u8; (width * width) as usize];

        let direction = 1u8;

        D1caUniverse {
            width,
            order,
            cells,
            direction,
            lattice,
        }
    }

    // 状態を変更して、初期化する
    pub fn renew(&mut self, width: u32, order: u32) {
        self.width = width;
        self.order = order;
        // self.direction = 1u8;
        self.cells = vec![0 as u8; width as usize];
        self.lattice = vec![0 as u8; (width * width) as usize];
        for idx in 0..width {
            let rand_number = rand::random::<u8>() % 2;
            self.cells[idx as usize] = rand_number;
            self.lattice[idx as usize] = rand_number;
        }
    }

    pub fn change_direction(&mut self) {
        self.direction = (self.direction + 1) % 3;
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_direction(&self) -> u8 {
        self.direction
    }
    pub fn get_cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
    pub fn get_lattice(&self) -> *const u8 {
        self.lattice.as_ptr()
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Lattice を描画する
impl fmt::Display for D1caUniverse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.lattice.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '□' } else { '■' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub type Universe = D1caUniverse;
