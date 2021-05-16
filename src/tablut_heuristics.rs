use crate::game::*;
use crate::heuristic::Heuristic;
use crate::tablut::*;

pub struct BuggedHeuristic;
impl Heuristic<Tablut> for BuggedHeuristic {
	fn eval(g: &Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let nd = g.d.count_ones() as i64;
				let na = g.a.count_ones() as i64;
				let mut km = 0i64;
				let kp = g.k.trailing_zeros();
				let capturer = g.a | CAPTURE_AID;
				let pass = !(g.a | g.d | BLOCK);

				let mut i = kp;
				while (pass >> i) & 1 != 0 {
					i += 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				//i = kp + 11;
				while (pass >> i) & 1 != 0 {
					i += 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				//i = kp - 1;
				while (pass >> i) & 1 != 0 {
					i -= 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				//i = kp - 11;
				while (pass >> i) & 1 != 0 {
					i -= 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				nd * 16 + km * 4 - na * 32 - (g.turn & 1) as i64
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
				const DIST: [i64; 121] = [
					6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 1, 0, 0, 1, 2, 1, 0, 0, 1, 6, 6, 0, 1, 1, 2, 3, 2, 1,
					1, 0, 6, 6, 0, 1, 2, 3, 4, 3, 2, 1, 0, 6, 6, 1, 2, 3, 4, 5, 4, 3, 2, 1, 6, 6, 2, 3, 4, 5,
					6, 5, 4, 3, 2, 6, 6, 1, 2, 3, 4, 5, 4, 3, 2, 1, 6, 6, 0, 1, 2, 3, 4, 3, 2, 1, 0, 6, 6, 0,
					1, 1, 2, 3, 2, 1, 1, 0, 6, 6, 1, 0, 0, 1, 2, 1, 0, 0, 1, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
					6,
				];
				let nd = g.d.count_ones() as i64; // number of defender pawns
				let na = g.a.count_ones() as i64; // number of attacker pawns
				let kp = g.k.trailing_zeros();
				let km = 6i64 - DIST[kp as usize] as i64; // 6 - king distance from edge
				let ks =
					(1u128 << (kp + 1)) | (1u128 << (kp + 11)) | (1u128 << (kp - 1)) | (1u128 << (kp - 11));
				let capturers = g.a | CAPTURE_AID;
				let kcs = (ks & capturers).count_ones() as i64; // king captured sides
				let vp = 0i64; // victory paths (unimplemented for now)
				250 * nd + 195 * vp + 42 * km - 147 * kcs - 164 * na
			}
		}
	}
}
