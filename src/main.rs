#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

extern crate env_logger;
extern crate fera;
extern crate mstcc;
extern crate rand;

// system
use std::time::Instant;

// external
use fera::ext::VecExt;
use fera::fun::vec;
use fera::graph::kruskal::Kruskal;
use fera::graph::prelude::*;

// local
use mstcc::*;

pub fn main() {
    let start = Instant::now();
    let args = args();

    init_logger(&args.log_level);

    info!("Using {:?}", args.seed);

    let mut p = read_sammer_urrutia(&args.file).unwrap();
    let mut rng = args.seed.new_xor_shift_rng();
    p.alpha = args.greedy_alpha;
    p.beta = args.greedy_beta;

    let mut tree = match args.init.as_str() {
        "random" => vec(p.g.kruskal().edges(vec(p.g.edges()).shuffled(&mut rng))),
        "kruskal" => vec(p.g.kruskal().weight(&p.w)),
        "greedy" => new_greedy(&p),
        _ => unreachable!(),
    };

    p.alpha = args.alpha;
    p.beta = args.beta;

    let mut ils = Ils {
        p: &p,
        max_iters: args.ils_max_iters,
        max_iters_no_improv: args.ils_max_iters_no_improv,
        num_excludes: args.ils_excludes,
        stop_on_feasible: args.stop_on_feasible,
    };

    let conflicts = match args.alg.as_str() {
        "2ex" => OneEdgeReplacement::new(&p).run(&mut tree),
        "4ex" => TwoEdgeReplacement::new(&p).run(&mut tree),
        "ils-2ex" => {
            let mut one = OneEdgeReplacement::new(&p);
            one.sort = args.sort;
            one.stop_on_feasible = args.stop_on_feasible;
            ils.run(&mut tree, &mut rng, |tree| one.run(tree))
        }
        "ils-4ex" => {
            let mut two = TwoEdgeReplacement::new(&p);
            two.sort = args.sort;
            two.stop_on_feasible = args.stop_on_feasible;
            ils.run(&mut tree, &mut rng, |tree| two.run(tree))
        }
        _ => unreachable!(),
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
    alpha: u32,
    beta: u32,
    greedy_alpha: u32,
    greedy_beta: u32,
    sort: bool,
    stop_on_feasible: bool,
    ils_max_iters: u32,
    ils_max_iters_no_improv: u32,
    ils_excludes: u32,
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
        (@arg alpha: --alpha
            default_value("1")
            "The alpha value used in objective function")
        (@arg beta: --beta
            default_value("10000")
            "The beta value used in objective function")
        (@arg greedy_alpha: --("greedy-alpha")
            default_value("1")
            "The alpha value used in objective function in the greedy init heuristic")
        (@arg greedy_beta: --("greedy-beta")
            default_value("10000")
            "The beta value used in objective function in the greedy init heuristic")
        (@arg ils_max_iters: --("ils-max-iters")
            default_value("100")
            "Maximum number of iterations for the ils algorithm")
        (@arg ils_max_iters_no_improv: --("ils-max-iters-no-improv")
            default_value("10")
            "Maximum number of iterations without improvement for the ils algorithm")
        (@arg ils_excludes: --("ils-excludes")
            default_value("1")
            "Number of edges to exclude in the perturbation phase of the ils algorithm")
        (@arg sort: --sort
            "Sort the edges in 2ex")
        (@arg stop_on_feasible: --("stop-on-feasible")
            "Stop when the first feasible solution is found")
        (@arg init: +required
            possible_value("random")
            possible_value("kruskal")
            possible_value("greedy")
            "The method used to create the initial solution")
        (@arg alg: +required
            possible_value("2ex")
            possible_value("4ex")
            possible_value("ils-2ex")
            possible_value("ils-4ex")
            "The algorithm to run")
        (arg: arg_input())
    );

    let matches = app.get_matches();

    Args {
        seed: value_t!(matches, "seed", Seed).unwrap_or_else(|_| Seed::new_random()),
        log_level: matches.value_of("level").unwrap().into(),
        alpha: value_t_or_exit!(matches, "alpha", u32),
        beta: value_t_or_exit!(matches, "beta", u32),
        init: matches.value_of("init").unwrap().into(),
        greedy_alpha: value_t_or_exit!(matches, "greedy_alpha", u32),
        greedy_beta: value_t_or_exit!(matches, "greedy_beta", u32),
        sort: matches.is_present("sort"),
        stop_on_feasible: matches.is_present("stop_on_feasible"),
        ils_max_iters: value_t_or_exit!(matches, "ils_max_iters", u32),
        ils_max_iters_no_improv: value_t_or_exit!(matches, "ils_max_iters_no_improv", u32),
        ils_excludes: value_t_or_exit!(matches, "ils_excludes", u32),
        alg: matches.value_of("alg").unwrap().into(),
        file: matches.value_of("input").unwrap().into(),
    }
}
