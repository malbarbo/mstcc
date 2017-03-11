// system
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

// external
use fera::graph::prelude::*;

// local
use MstCcProblem;

pub fn read_sammer_urrutia(file: &str) -> Result<MstCcProblem, Box<Error>> {
    debug!("Start read_sammer_urrutia: {}", file);

    let lines = &mut BufReader::new(File::open(file)?)
        .lines()
        .map(Result::unwrap)
        .skip_while(|s| s.starts_with('#'));

    let name = lines.next().unwrap();
    let n: usize = parse_next(lines)?;
    let m: usize = parse_next(lines)?;
    let c: usize = parse_next(lines)?;

    let mut b = <StaticGraph as WithBuilder>::Builder::new(n, m);
    let mut w = Vec::<u32>::new();

    for _ in 0..m {
        let line = lines.next().unwrap();
        let s = &mut line.split_whitespace();
        b.add_edge(parse_next(s)?, parse_next(s)?);
        w.push(parse_next(s)?);
    }

    let g = b.finalize();
    let mut cc = g.default_edge_prop(vec![]);
    let mut num_cc = 0;

    for _ in 0..c {
        let line = lines.next().unwrap();
        let s = &mut line.split_whitespace();
        let (a, b) = (parse_next(s)?, parse_next(s)?);
        let (x, y) = (parse_next(s)?, parse_next(s)?);
        let ab = g.edge_by_ends(a, b);
        let xy = g.edge_by_ends(x, y);
        cc[ab].push(xy);
        cc[xy].push(ab);
        num_cc += 1;
    }

    let mut i = 0;
    let w = g.default_edge_prop_from_fn(|e| {
        assert_eq!(g.edge_index().get(e), i);
        let x = w[i];
        i += 1;
        x
    });

    assert_eq!(n, g.num_vertices());
    assert_eq!(m, g.num_edges());
    assert_eq!(c, num_cc);

    info!("n = {}, m = {}, cc = {}", n, m, num_cc);

    debug!("End read_sammer_urrutia: {}", file);

    Ok(MstCcProblem {
        name: name,
        g: g,
        w: w,
        cc: cc,
        num_cc: num_cc,
    })
}

fn parse_next<I, T>(iter: &mut I) -> Result<T, T::Err>
    where I: Iterator,
          I::Item: AsRef<str>,
          T: FromStr
{
    iter.next().unwrap().as_ref().parse()
}
