#[macro_use]
extern crate clap;

extern crate env_logger;
extern crate fera;
extern crate mstcc;
extern crate rand;

// system
use std::time::Instant;

// external
use fera::fun::vec;
use fera::graph::kruskal::Kruskal;
use fera::graph::prelude::*;
use fera::ext::VecExt;

// local
use mstcc::*;

const ALFA: u32 = 10_000;

pub fn main() {
    let start = Instant::now();
    let args = args();

    init_logger(&args.log_level);

    let p = read_sammer_urrutia(&args.file).unwrap();
    let mut rng = args.seed.new_xor_shift_rng();

    let mut tree = match args.init.as_str() {
        "random" => vec(p.g.kruskal().edges(vec(p.g.edges()).shuffled(&mut rng))),
        "kruskal" => vec(p.g.kruskal().weight(&p.w)),
        "greedy" => new_greedy(&p),
        _ => unreachable!()
    };

    let conflicts = match args.alg.as_str() {
        "2ex" => {
            // TODO: Rename OneEdgeReplacement
            let mut one = OneEdgeReplacement::new(&p);
            one.run(&mut tree, 1, ALFA)
        }
        "4ex" => two_opt(&p, &mut tree, &[], 1, ALFA),
        _ => unreachable!()
    };

    let elapsed = start.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

    let weight: u32 = sum_prop(&p.w, &tree);
    print!("{},{:.02},{},{},", p.name, elapsed, conflicts, weight);
    for (u, v) in p.g.ends(tree) {
        print!("{}-{} ", u, v);
    }
    println!();
}

struct Args {
    seed: Seed,
    log_level: String,
    init: String,
    alg: String,
    file: String,
}

fn args() -> Args {
    let app = clap_app!(("mstcc") =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "mstcc solver based on {2, 4}-exchange neighborhood")
        (arg: arg_seed())
        (arg: arg_log())
        (@arg init: +required
            possible_value[random kruskal heuristic]
            "The method used to create the initial solution")
        (@arg alg: +required
            possible_value("2ex")
            possible_value("4ex")
            "The algorithm to run")
        (arg: arg_input())
    );

    let matches = app.get_matches();

    Args {
        seed: value_t!(matches, "seed", Seed).unwrap_or_else(|_| Seed::new_random()),
        log_level: matches.value_of("level").unwrap().into(),
        init: matches.value_of("init").unwrap().into(),
        alg: matches.value_of("alg").unwrap().into(),
        file: matches.value_of("input").unwrap().into(),
    }
}
