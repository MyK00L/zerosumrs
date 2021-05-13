use crate::game::*;
use crate::heuristic::Heuristic;
use crate::tablut::*;

pub struct BmHeuristic;
impl Heuristic<Tablut> for BmHeuristic {
	fn eval(g: &Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX,
			State::Lose => i64::MIN,
			State::Draw => 0,
			State::Going => {
				const RHOMBUS: [u8; 8] = [
					1 * 9 + 2,
					1 * 9 + 6,
					2 * 9 + 1,
					2 * 9 + 7,
					6 * 9 + 1,
					6 * 9 + 7,
					7 * 9 + 2,
					7 * 9 + 6,
				];
				let mut nb = 0i64;
				let mut nr = 0i64;
				let mut nw = 0i64;
				let mut sur = 0i64;
				for i in 0..81 {
					match g.get(i) {
						Tile::D => {
							nw += 1;
						}
						Tile::A => {
							nb += 1;
						}
						Tile::K => {
							if g.get(i - 9) == Tile::A {
								sur += 1;
							}
							if g.get(i - 1) == Tile::A {
								sur += 1;
							}
							if g.get(i + 1) == Tile::A {
								sur += 1;
							}
							if g.get(i + 9) == Tile::A {
								sur += 1;
							}
						}
						Tile::E => {}
					}
				}
				if nb >= 10 {
					for p in RHOMBUS.iter() {
						if g.get(*p) == Tile::A {
							nr += 1;
						}
					}
				}
				let bv = nb * 35 + (8 - nw) * 48 * 2 + sur * 15 * 4 + nr;
				let wv = 0;
				wv - bv
			}
		}
	}
}
pub struct FmHeuristic;
impl Heuristic<Tablut> for FmHeuristic {
	fn eval(g: &Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				const DIST: [i64;81] =
				[
				1, 0, 0, 1, 2, 1, 0, 0, 1,
				0, 1, 1, 2, 3, 2, 1, 1, 0,
				0, 1, 2, 3, 4, 3, 2, 1, 0,
				1, 2, 3, 4, 5, 4, 3, 2, 1,
				2, 3, 4, 5, 6, 5, 4, 3, 2,
				1, 2, 3, 4, 5, 4, 3, 2, 1,
				0, 1, 2, 3, 4, 3, 2, 1, 0,
				0, 1, 1, 2, 3, 2, 1, 1, 0,
				1, 0, 0, 1, 2, 1, 0, 0, 1,
				];

				let mut nd = 0; // number of defender pawns
				let mut na = 0; // number of attacker pawns
				let mut km = 0; // 6 - king distance from edge
				let mut kcs = 0; // king captured sides
				let mut vp = 4; // victory paths
				for i in 0..81 {
					match g.get(i) {
						Tile::D => {nd+=1;},
						Tile::A => {na+=1;},
						Tile::K => {
							km = 6-DIST[i as usize];
							if g.get(i - 9) == Tile::A || is_capture_aid(i-9) {
								kcs += 1;
							}
							if g.get(i - 1) == Tile::A || is_capture_aid(i-1) {
								kcs += 1;
							}
							if g.get(i + 1) == Tile::A || is_capture_aid(i+1) {
								kcs += 1;
							}
							if g.get(i + 9) == Tile::A || is_capture_aid(i+9) {
								kcs += 1;
							}
							let mut p = i;
							while p%9>0 {
								p-=1;
								let t = g.get(p);
								if t==Tile::A || t==Tile::D || is_capture_aid(p) {
									vp-=1;
									break;
								}
							}
							p=i;
							while p>8 {
								p-=9;
								let t = g.get(p);
								if t==Tile::A || t==Tile::D || is_capture_aid(p) {
									vp-=1;
									break;
								}
							}
							p=i;
							while p%9<8 {
								p+=1;
								let t = g.get(p);
								if t==Tile::A || t==Tile::D || is_capture_aid(p) {
									vp-=1;
									break;
								}
							}
							p=i;
							while p<9*8 {
								p+=9;
								let t = g.get(p);
								if t==Tile::A || t==Tile::D || is_capture_aid(p) {
									vp-=1;
									break;
								}
							}
						},
						_ => {}
					}
				}
				250*nd+195*vp+42*km-147*kcs-164*na
			}
		}
	}
}

pub struct MyHeuristic;
impl Heuristic<Tablut> for MyHeuristic {
	fn eval(g: &Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let mut nd = 0; // number of defender pawns
				let mut na = 0; // number of attacker pawns
				let mut ks = 0; // king safety
				for i in 0..81 {
					match g.get(i) {
						Tile::D => {nd+=1;},
						Tile::A => {na+=1;},
						Tile::K => {
							if i==4*9+4 || i==4*9+5 || i==4*9+3 || i==3*9+4 || i==5*9+4 {
								ks+=3;
							}
							let mut p = i;
							while p%9 > 0 {
								p-=1;
								let t = g.get(p);
								ks+=1;
								if t==Tile::A || is_capture_aid(p) {
									ks-=1;
									break;
								}
								if t==Tile::D {
									break;
								}
							}
							p=i;
							while p > 8 {
								p-=9;
								let t = g.get(p);
								ks+=1;
								if t==Tile::A || is_capture_aid(p) {
									ks-=1;
									break;
								}
								if t==Tile::D {
									break;
								}
							}
							p=i;
							while p%9 < 8 {
								p+=1;
								let t = g.get(p);
								ks+=1;
								if t==Tile::A || is_capture_aid(p) {
									ks-=1;
									break;
								}
								if t==Tile::D {
									break;
								}
							}
							p=i;
							while p<9*8 {
								p+=9;
								let t = g.get(p);
								ks+=1;
								if t==Tile::A || is_capture_aid(p) {
									ks-=1;
									break;
								}
								if t==Tile::D {
									break;
								}
							}
						},
						Tile::E => {},
					}
				}
				nd*32-na*16+ks-4
			}
		}
	}
}
pub struct CountHeuristic;
impl Heuristic<Tablut> for CountHeuristic {
	fn eval(g: &Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let mut ans = 0;
				for i in 0..81 {
					match g.get(i) {
						Tile::D => {ans+=2;},
						Tile::A => {ans-=1;},
						_ => {}
					}
				}
				ans
			},
		}
	}
}

