use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
	buffer: Vec<String>,
}
impl Scanner {
	fn next<T: std::str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parse");
			}
			let mut input = String::new();
			stdin().read_line(&mut input).expect("Failed read");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

mod ai;
mod game;
mod mancala;
mod minimax_hard;
mod minimax_simple;
mod monte_carlo;
mod monte_carlo_total;
mod monte_carlo_tree_search;
mod othello;
mod tictactoe;

use ai::*;
use game::*;
use mancala::Mancala;
use minimax_hard::*;
use minimax_simple::*;
use monte_carlo::*;
use monte_carlo_total::*;
use monte_carlo_tree_search::*;
use othello::Othello;
use tictactoe::Ttt;

fn random_play<G: Game>() -> State {
	let mut rng = Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap();
	let mut g = G::new(true);
	while g.state() == State::Going {
		let moves = g.get_moves();
		let m = moves.choose(&mut rng).unwrap();
		g.mov(&m);
	}
	g.state()
}

fn print_balance<G: Game>() {
	let mut nw = 0;
	let mut nl = 0;
	let mut nd = 0;
	let mut ne = 0;
	for i in 0..1000 {
		match random_play::<G>() {
			State::Win => {
				nw += 1;
			}
			State::Lose => {
				nl += 1;
			}
			State::Draw => {
				nd += 1;
			}
			_ => {
				ne += 1;
			}
		}
	}
	eprintln!("w {}\nl {}\nd {}\ne {}", nw, nl, nd, ne);
}

fn test_rollback<G: Game>() {
	let mut rng = Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap();
	let mut g = G::new(true);
	let mut v = vec![g.clone()];
	while g.state() == State::Going {
		let moves = g.get_moves();
		let m = moves.choose(&mut rng).unwrap();
		g.mov(&m);
		eprintln!("{}", g);
		v.push(g.clone());
	}
	eprintln!("rolling");
	while !v.is_empty() {
		let x = v.pop().unwrap();
		if x.get_static_state() != g.get_static_state() {
			eprintln!("rollback test failed!");
			return;
		}
		if !v.is_empty() {
			g.rollback();
		}
		eprintln!("{}", g);
	}
}

fn main() {
	let mut scan = Scanner::default();
	let out = &mut BufWriter::new(stdout());
	let x = compete::<Othello, MinimaxHard<Othello>, MinimaxSimple<Othello>>();
	write!(out, "{:?}", x);
	//print_balance::<Othello>();
}
