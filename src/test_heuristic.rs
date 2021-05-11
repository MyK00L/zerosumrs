use crate::game::*;
use crate::heuristic::Heuristic;
use crate::mancala;
use crate::othello;
use crate::tablut;
use crate::tictactoe;

pub struct TestHeuristic;

impl Heuristic<tablut::Tablut> for TestHeuristic {
	fn eval(g: &tablut::Tablut) -> i64 {
		const KV: [i64; 81] = [
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 1, 0, 1, 6, 6, 0, 0, 6, 8, 2, 1, 2, 8, 6, 0, 0, 1, 2, 2,
			4, 2, 2, 1, 0, 0, 0, 1, 4, 4, 4, 1, 0, 0, 0, 1, 2, 2, 4, 2, 2, 1, 0, 0, 6, 8, 2, 1, 2, 8, 6,
			0, 0, 6, 6, 1, 0, 1, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		];
		match g.state() {
			State::Win => 32768 - g.turn as i64,
			State::Lose => -32768 + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let mut nd = 0;
				let mut na = 0;
				let mut ma = 0;
				let mut md = 0;
				let mut mk = 0;
				let mut kp = 0;
				for i in 0..81 {
					let t = g.get(i);
					if t == tablut::Tile::D {
						nd += 1;
					}
					if t == tablut::Tile::A {
						na += 1;
					}
					if t == tablut::Tile::K {
						kp = i as usize;
					}
				}
				// right
				for y in 0..9 {
					let mut last = tablut::Tile::E;
					let mut lastp = 128u8;
					for x in 0..9 {
						let p = tablut::mapc(x, y);
						let t = g.get(p);
						if t == tablut::Tile::E {
							if tablut::is_block_um(p)
								&& (last == tablut::Tile::E || !tablut::is_block_um(lastp) || p - lastp > 2)
							{
								last = tablut::Tile::E;
							} else if last != tablut::Tile::E {
								match last {
									tablut::Tile::D => {
										md += 1;
									}
									tablut::Tile::K => {
										mk += 1;
									}
									tablut::Tile::A => {
										ma += 1;
									}
									_ => {}
								}
							}
						} else {
							last = t;
							lastp = tablut::mapc(x, y);
						}
					}
				}
				// left
				for y in 0..9 {
					let mut last = tablut::Tile::E;
					let mut lastp = 128u8;
					for x in (0..9).rev() {
						let p = tablut::mapc(x, y);
						let t = g.get(p);
						if t == tablut::Tile::E {
							if tablut::is_block_um(p)
								&& (last == tablut::Tile::E || !tablut::is_block_um(lastp) || lastp - p > 2)
							{
								last = tablut::Tile::E;
							} else if last != tablut::Tile::E {
								match last {
									tablut::Tile::D => {
										md += 1;
									}
									tablut::Tile::K => {
										mk += 1;
									}
									tablut::Tile::A => {
										ma += 1;
									}
									_ => {}
								}
							}
						} else {
							last = t;
							lastp = tablut::mapc(x, y);
						}
					}
				}
				// down
				for x in 0..9 {
					let mut last = tablut::Tile::E;
					let mut lastp = 128u8;
					for y in 0..9 {
						let p = tablut::mapc(x, y);
						let t = g.get(p);
						if t == tablut::Tile::E {
							if tablut::is_block_um(p)
								&& (last == tablut::Tile::E || !tablut::is_block_um(lastp) || p - lastp > 2 * 9)
							{
								last = tablut::Tile::E;
							} else if last != tablut::Tile::E {
								match last {
									tablut::Tile::D => {
										md += 1;
									}
									tablut::Tile::K => {
										mk += 1;
									}
									tablut::Tile::A => {
										ma += 1;
									}
									_ => {}
								}
							}
						} else {
							last = t;
							lastp = tablut::mapc(x, y);
						}
					}
				}
				// up
				for x in 0..9 {
					let mut last = tablut::Tile::E;
					let mut lastp = 128u8;
					for y in (0..9).rev() {
						let p = tablut::mapc(x, y);
						let t = g.get(p);
						if t == tablut::Tile::E {
							if tablut::is_block_um(p)
								&& (last == tablut::Tile::E || !tablut::is_block_um(lastp) || lastp - p > 2 * 9)
							{
								last = tablut::Tile::E;
							} else if last != tablut::Tile::E {
								match last {
									tablut::Tile::D => {
										md += 1;
									}
									tablut::Tile::K => {
										mk += 1;
									}
									tablut::Tile::A => {
										ma += 1;
									}
									_ => {}
								}
							}
						} else {
							last = t;
							lastp = tablut::mapc(x, y);
						}
					}
				}
				//let mut ans = -16 + if g.turn() { 1 } else { -1 };
				//nd * 6 - na * 3 - ma + 2 * md + 4 * mk
				nd * 24 - na * 12 - ma + 2 * md + 4 * mk - 16 + if g.turn() { 1 } else { -1 } + KV[kp] * 4
			}
		}
	}
}
/*
impl Heuristic<tictactoe::Tictactoe> for DefaultHeuristic {
	fn eval(g: &tictactoe::Tictactoe) -> i64 {
		match g.state() {
			State::Win => 1,
			State::Lose => -1,
			_ => 0,
		}
	}
}

impl Heuristic<othello::Othello> for DefaultHeuristic {
	fn eval(g: &othello::Othello) -> i64 {
		const WEIGHTS: [i64; 64] = [
			4, -3, 2, 2, 2, 2, -3, 4, -3, -4, -1, -1, -1, -1, -4, -3, 2, -1, 1, 0, 0, 1, -1, 2, 2, -1, 0,
			1, 1, 0, -1, 2, 2, -1, 0, 1, 1, 0, -1, 2, 2, -1, 1, 0, 0, 1, -1, 2, -3, -4, -1, -1, -1, -1,
			-4, -3, 4, -3, 2, 2, 2, 2, -3, 4,
		];
		match g.state() {
			State::Win => 256,
			State::Lose => -256,
			State::Draw => 0,
			State::Going => {
				let mut ans = 0i64;
				for i in 0..64 {
					ans += if g.has_piece(i) {
						if g.get_piece(i) {
							WEIGHTS[i as usize]
						} else {
							-WEIGHTS[i as usize]
						}
					} else {
						0
					};
				}
				ans
			}
		}
	}
}

impl Heuristic<mancala::Mancala> for DefaultHeuristic {
	fn eval(g: &mancala::Mancala) -> i64 {
		const WEIGHTS: [i64; 14] = [7, 6, 5, 4, 3, 2, 8, -7, -6, -5, -4, -3, -2, -8];
		match g.state() {
			State::Win => 32768,
			State::Lose => -32768,
			State::Draw => 0,
			_ => {
				let mut res = 0i64;
				for (i, wi) in WEIGHTS.iter().enumerate() {
					res += wi * g.a[i] as i64;
				}
				res
			}
		}
	}
}*/
