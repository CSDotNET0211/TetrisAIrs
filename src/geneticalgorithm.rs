//! 実数値遺伝アルゴリズム

use std::borrow::{Borrow, BorrowMut};

use rand::{prelude::ThreadRng, *};

use crate::evaluation::Evaluation;
#[derive(Copy, Clone)]
struct Indivisual {
    values: [f64; Evaluation::WEIGHT_COUNT as usize],
    evaluation: f64,
}

impl Indivisual {
    pub fn new() -> Self {
        Indivisual {
            evaluation: f64::MAX,
            values: [f64::MAX; Evaluation::WEIGHT_COUNT as usize],
        }
    }
}

struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn bench_mark_test() {
        let mut gen_count = 0;
        let random = rand::thread_rng();
        //   let mut indivisuals = Vec::new();

        for _i in 0..30 {
            //     let param=
        }
    }

    fn learn() {}

    fn bla_alpha_crossover() {}

    fn roulette_choise(rate: &mut [i32], random: &ThreadRng) -> i32 {
        let min = *rate.iter().min().unwrap();
        if min < 0 {
            for i in 0..rate.len() {
                rate[i] -= min - 1;
            }
        }

        2
    }

    fn tournament_choise(indivisuals: &[Indivisual], tournament_count: i32) -> Indivisual {
        let mut rng = rand::thread_rng();
        let mut temp = Vec::new();
        for i in 0..indivisuals.len() {
            temp.push(i);
        }

        let mut test = Vec::new();
        for _i in 0..tournament_count {
            test.push(rng.gen_range(0..temp.len()));
            temp.remove(test[test.len() - 1]);
        }

        let mut best = Indivisual::new();
        for i in 0..test.len() {
            if best.evaluation == f64::MAX || best.evaluation < indivisuals[test[i]].evaluation {
                best = indivisuals[test[i]];
            }
        }

        best
    }

    fn elite_choise(indivisuals: &[Indivisual]) -> Indivisual {
        let mut result = Indivisual::new();
        for indivisual in indivisuals {
            if result.evaluation == f64::MAX || indivisual.evaluation > result.evaluation {
                result = *indivisual;
            }
        }

        result
    }

    fn get_random(min: f64, max: f64, mut random: ThreadRng) {
        let range = max - min;
        let sample = random.gen_range(0..1);
    }
}
