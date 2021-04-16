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
mod tablut;
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
use tablut::Tablut;
use tictactoe::Ttt;

fn random_play<G: Game>() -> (State, usize) {
	let mut rng = Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap();
	let mut g = G::new(true);
	let mut nmovs = 0;
	while g.state() == State::Going {
		let moves = g.get_moves();
		let m = moves.choose(&mut rng).unwrap();
		g.mov(&m);
		nmovs += 1;
	}
	(g.state(), nmovs)
}

fn print_stats<G: Game>() {
	let mut nw = 0;
	let mut nl = 0;
	let mut nd = 0;
	let mut ne = 0;
	let mut al = 0;
	for i in 0..1000 {
		let cacca = random_play::<G>();
		al += cacca.1;
		match cacca.0 {
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
	eprintln!(
		"win {}\nlos {}\ndrw {}\nerr {}\navg len {}",
		nw,
		nl,
		nd,
		ne,
		al / 1000
	);
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
	print_stats::<Tablut>();
	let x = compete::<Tablut, MonteCarloTreeSearch<Tablut>, MonteCarloTreeSearch<Tablut>>();
	write!(out, "{:?}", x);
}
