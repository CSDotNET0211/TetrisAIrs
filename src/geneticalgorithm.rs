//! 実数値遺伝アルゴリズム

use rand::{prelude::ThreadRng, *};

struct Indivisual {
    values: [f64; 0],
    evaluation: f64,
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

    fn roulette_choise() {}

    fn tournament_choise() {}

    fn elite_choise() {}

    fn get_random(min: f64, max: f64, mut random: ThreadRng) {
        let range = max - min;
        let sample = random.gen_range(0..1);
    }
}
