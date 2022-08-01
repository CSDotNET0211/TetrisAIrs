//! 実数値遺伝アルゴリズム

use core::panic;
use std::{
    f64::consts::PI,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    ops::{Index, IndexMut},
};

use rand::{prelude::ThreadRng, *};

use crate::{environment::Environment, evaluation::Evaluation};
#[derive(Copy, Clone)]
struct Indivisual {
    pub values: [f64; Evaluation::WEIGHT_COUNT as usize],
    pub evaluation: f64,
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
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
                Self::get_random(-5.12, -2.12, &mut random),
            ];
            indivisuals.push(Indivisual {
                evaluation: Self::Function(&param),
                values: param,
            })
        }

        let mut gen_count = 0;
        loop {
            indivisuals.sort_by(|a, b| a.evaluation.partial_cmp(&b.evaluation).unwrap());

            println!("gen:{}", gen_count);
            for i in 0..indivisuals.len() {
                println!(
                    "値:{}:{},評価:{}",
                    indivisuals[i].values[0], indivisuals[i].values[1], indivisuals[i].evaluation
                );
            }
            println!("---------------");

            let index1 = random.gen_range(0..indivisuals.len());
            let index2 = random.gen_range(0..indivisuals.len());

            if index1 == index2 {
                continue;
            }

            //子作り
            let mut childs = Vec::new();
            for _i in 0..20 {
                childs.push(Self::bla_alpha_crossover(
                    &indivisuals.index(index1),
                    &indivisuals.index(index2),
                    0.5,
                ));
            }

            for i in 0..20 {
                childs.index_mut(i as usize).evaluation = Self::Function(&childs.index(i).values);
            }

            childs.push(indivisuals[index1].clone());
            childs.push(indivisuals[index2].clone());

            //選別
            let elite = Self::elite_choise(&childs);
            let roulette = Self::roulette_choise1(&childs);
            let roulette = childs[roulette as usize].clone();

            *indivisuals.index_mut(index1) = elite;
            *indivisuals.index_mut(index2) = roulette;

            gen_count += 1;
        }
    }

    fn Function(array: &[f64]) -> f64 {
        let mut result = 0.0;
        for i in 0..array.len() {
            result += (array[i].powf(2.0) as f64
                - 10 as f64 * (2 as f64 * PI * array[i] as f64).cos()
                + 10 as f64)
        }

        result
    }

    pub fn learn() {
        let mut input = String::new();
        let indivisual_count;
        let child_count;
        let random_max;
        let random_min;

        println!("前学習データを読み込みますか？");
        println!("1 Yes");
        println!("2 No");
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {}
            "2" => {}
            _ => panic!("不明な操作"),
        }

        input = "".to_string();
        println!("世代数を入力");
        io::stdin().read_line(&mut input).unwrap();
        indivisual_count = input.trim().parse::<i32>().unwrap();

        input = "".to_string();
        println!("子供数を入力");
        io::stdin().read_line(&mut input).unwrap();
        child_count = input.trim().parse::<i32>().unwrap();

        input = "".to_string();
        println!("初期化値の上限");
        io::stdin().read_line(&mut input).unwrap();
        random_max = input.trim().parse::<f64>().unwrap();

        input = "".to_string();
        println!("初期化値の下限");
        io::stdin().read_line(&mut input).unwrap();
        random_min = input.trim().parse::<f64>().unwrap();

        println!("MGGで学習を開始します\r\n学習結果は[index].txtとして保存されます");

        let mut gen_count = 0;
        let mut random = rand::thread_rng();

        let mut indivisuals = Vec::new();
        for _i in 0..indivisual_count {
            let mut param = [0.0; Evaluation::WEIGHT_COUNT as usize];
            for j in 0..param.len() {
                param[j] = Self::get_random(random_min, random_max, &mut random);
            }

            indivisuals.push(Indivisual {
                values: param,
                evaluation: Environment::get_eval(&param) as f64,
            })
        }

        loop {
            println!("{}世代", gen_count);

            indivisuals.sort_by(|a, b| b.evaluation.partial_cmp(&a.evaluation).unwrap());

            println!();

            for i in 0..indivisuals.len() {
                print!("{}番目の評価:{}\r\n重み", i + 1, indivisuals[i].evaluation);
                for j in 0..indivisuals[i].values.len() {
                    print!("{} ", indivisuals[i].values[j]);
                }
            }
            print!("\r\n");

            /*   if gen_count % 10 == 0 {
                let mut file = File::create(gen_count.to_string() + ".txt");

            match file {
                    Ok(file) => {
                        for indivisual in indivisuals {
                            for value in indivisual.values {
                                file.write(value.to_string().as_bytes_mut());
                            }
                            file.write(b";");
                        }
                    }
                    Err(e) => {}
                }
            }*/

            let mut childs = Vec::new();

            let mut parent1index;
            let mut parent2index;

            while {
                parent1index = random.gen_range(0..indivisuals.len());
                parent2index = random.gen_range(0..indivisuals.len());

                parent1index == parent2index
            } {}

            for _child_index in 0..child_count {
                childs.push(Self::bla_alpha_crossover(
                    indivisuals.index(parent1index),
                    indivisuals.index(parent2index),
                    0.5,
                ));

                childs.index_mut(childs.len() - 1).evaluation =
                    Environment::get_eval(&childs.index(childs.len() - 1).values) as f64;
            }

            childs.push(indivisuals.index(parent1index).clone());
            childs.push(indivisuals.index(parent2index).clone());

            //評価しろ
            let elite = Self::elite_choise(&childs);
            let roulette = Self::roulette_choise1(&childs) as usize;

            *indivisuals.index_mut(parent1index) = elite;
            *indivisuals.index_mut(parent2index) = *childs.index(roulette);

            gen_count += 1;
        }
    }

    fn bla_alpha_crossover(
        indivisual1: &Indivisual,
        indivisual2: &Indivisual,
        alpha: f64,
    ) -> Indivisual {
        let mut random = rand::thread_rng();

        let mut child = Indivisual::new();
        child.values = [0.0; Evaluation::WEIGHT_COUNT as usize];

        for i in 0..Evaluation::WEIGHT_COUNT as usize {
            let mut x_max;
            let mut x_min;
            if indivisual1.values[i] < indivisual2.values[i] {
                x_max = indivisual2.values[i];
                x_min = indivisual1.values[i];
            } else {
                x_max = indivisual1.values[i];
                x_min = indivisual2.values[i];
            }

            let dx = (x_min - x_max).abs() * alpha;
            x_max += dx;
            x_min -= dx;

            child.values[i] = Self::get_random(x_min, x_max, &mut random);
        }

        child
    }

    fn roulette_choise1(indivisuals: &[Indivisual]) -> i32 {
        let mut random = rand::thread_rng();

        let mut test = Vec::new();
        for i in 0..indivisuals.len() {
            test.push((indivisuals[i].evaluation * 100 as f64) as i32)
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

        if max == 0 {
            panic!("?");
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

    ///小さいほうを選ぶ
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
