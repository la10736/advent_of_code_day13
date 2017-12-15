#![feature(conservative_impl_trait)]

use std::io::prelude::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or(String::from("example"));
    let content = read_all(fname);

    let firewall = firewall(&content);

    let score = matches_score(&firewall, 0);
    let delay = clean_delay(&firewall);

    println!("Score {}", score);
    println!("Delay {}", delay);
}

fn matches<'a>(firewall: &'a [(usize, usize)], delay: usize) -> impl Iterator<Item=(usize, usize)> + 'a {
    firewall.into_iter().filter(move |&&(p, d)| (p + delay) % (d + d - 2) == 0)
        .map(|c| c.clone())
}

fn matches_score<V: AsRef<[(usize, usize)]>>(firewall: V, delay: usize) -> usize {
    matches(firewall.as_ref(), delay).map(|(p, d)| p * d).sum()
}

fn wall(l: &str) -> (usize, usize) {
    let mut tokens = l.splitn(2, ": ");
    (tokens.next().unwrap().parse().unwrap(), tokens.next().unwrap().parse().unwrap())
}

fn firewall(data: &str) -> Vec<(usize, usize)> {
    data.lines().map(|l| wall(l)).collect()
}

fn clean_delay<V: AsRef<[(usize, usize)]>>(firewall: V) -> usize {
    (0..).into_iter()
        .filter(|d| matches(firewall.as_ref(), (d.clone() as usize)).next().is_none())
        .nth(0).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matches_score() {
        let firewall = vec![(0, 4), (1, 3), (2, 4), (5, 4), (6, 4)];

        assert_eq!(4 * 6, matches_score(&firewall, 0))
    }

    #[test]
    fn test_matches() {
        let firewall = vec![(0, 4), (1, 3), (2, 4), (5, 4), (6, 4)];

        let r = matches(&firewall, 0).collect::<Vec<_>>();
        assert_eq!(vec![(0, 4), (6, 4)], r)
    }

    static DATA: &'static str = "0: 3\n\
                                1: 2\n\
                                4: 4\n\
                                6: 4\
                                ";

    #[test]
    fn read_firewall() {
        let firewall = firewall(DATA);

        assert_eq!(vec![(0, 3), (1, 2), (4, 4), (6, 4)], firewall);
    }

    #[test]
    fn integration() {
        let firewall = firewall(DATA);

        assert_eq!(10, clean_delay(firewall));
    }
}
