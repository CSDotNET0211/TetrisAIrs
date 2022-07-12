//! 実数値遺伝アルゴリズム

use std::{
    borrow::{Borrow, BorrowMut},
    f64::consts::PI,
    ops::{Index, IndexMut},
};

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

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn bench_mark_test() {
        let mut gen_count = 0;
        let mut random = rand::thread_rng();
        let mut indivisuals = Vec::new();

        for _i in 0..50 {
            let param = [
                Self::get_random(-5.12, 5.12, &mut random),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ];
            indivisuals.push(Indivisual {
                evaluation: Self::Function(param[0]),
                values: param,
            })
        }

        loop {
            for i in 0..indivisuals.len() {
                println!("{},{}", indivisuals[i].values[0], indivisuals[i].evaluation);
            }
            println!("---------------");

            let index1 = random.gen_range(0..indivisuals.len());
            let index2 = random.gen_range(0..indivisuals.len());

            //子作り
            let mut childs = Vec::new();
            for _i in 0..20 {
                childs.push(Self::bla_alpha_crossover(
                    &indivisuals.index(index1),
                    &indivisuals.index(index2),
                    0.5,
                ));
            }

            childs.push(indivisuals[index1].clone());
            childs.push(indivisuals[index2].clone());

            //選別
            let elite = Self::elite_choise(&childs);
            let roulette = Self::roulette_choise1(&childs);
            let roulette = childs[roulette as usize].clone();

            *indivisuals.index_mut(index1) = elite;
            *indivisuals.index_mut(index2) = roulette;
        }
    }

    fn Function(x: f64) -> f64 {
        x.powf(2.0) as f64 - 10 as f64 * (2 as f64 * PI * x as f64).cos()
    }

    fn learn() {}

    fn bla_alpha_crossover(
        indivisual1: &Indivisual,
        indivisual2: &Indivisual,
        alpha: f64,
    ) -> Indivisual {
        let mut random = rand::thread_rng();

        let mut child = Indivisual::new();
        child.values = [0.0; Evaluation::WEIGHT_COUNT as usize];

        for i in 0..Evaluation::WEIGHT_COUNT as usize {
            let mut xMax;
            let mut xMin;
            if indivisual1.values[i] < indivisual2.values[i] {
                xMax = indivisual2.values[i];
                xMin = indivisual1.values[i];
            } else {
                xMax = indivisual1.values[i];
                xMin = indivisual2.values[i];
            }

            let dx = (xMin - xMax).abs() * alpha;
            xMax += dx;
            xMin -= dx;

            child.values[i] = Self::get_random(xMin, xMax, &mut random);
        }

        child
    }

    fn roulette_choise1(indivisuals: &[Indivisual]) -> i32 {
        let mut random = rand::thread_rng();

        let mut test = Vec::new();
        for i in 0..indivisuals.len() {
            test.push((indivisuals[i].evaluation * 10000 as f64) as i32)
        }

        let result = Self::roulette_choise(&mut test, &mut random);

        result
    }

    fn roulette_choise(rate: &mut Vec<i32>, random: &mut ThreadRng) -> i32 {
        let min = *rate.iter().min().unwrap();
        if min < 0 {
            for i in 0..rate.len() {
                rate[i] -= min - 1;
            }
        }

        let mut max = 0;
        for i in 0..rate.len() {
            max += rate[i];
        }

        let mut temp = random.gen_range(0..max);

        for i in 0..rate.len() {
            temp -= rate[i];
            if temp < 0 {
                return i as i32;
            }
        }

        panic!("なにこれ");
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

    fn get_random(min: f64, max: f64, random: &mut ThreadRng) -> f64 {
        let range = max - min;
        let sample = random.gen_range(0.0..1.0);
        let scaled = sample * range + min;
        let f = scaled;
        f
    }
}
