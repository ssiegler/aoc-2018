extern crate aoc_2018;

use std::str::FromStr;

use aoc_2018::file_lines;

fn main() {
    let claims: Vec<Claim> = read_claims();
    let cloth = claim_cloth(&claims);

    println!("overlap area: {}", cloth.overlap_area());
    println!("non overlapping claim: {}", non_overlapping(&cloth, &claims).id);
}

fn read_claims() -> Vec<Claim> {
    file_lines().map(|s| s.parse::<Claim>().unwrap()).collect()
}

fn claim_cloth(claims: &Vec<Claim>) -> Cloth {
    let width: usize = claims.iter().map(|r| r.right as usize).max().unwrap();
    let height: usize = claims.iter().map(|r| r.bottom as usize).max().unwrap();
    let mut cloth: Cloth = Cloth::new(width, height);
    for claim in claims {
        cloth.claim(claim);
    }
    cloth
}

fn non_overlapping<'a>(cloth: &Cloth, claims: &'a Vec<Claim>) -> &'a Claim {
    claims.iter().find(|&claim| cloth.has_no_overlap(claim)).unwrap()
}

#[derive(Debug)]
struct Cloth {
    width: usize,
    height: usize,
    coverage: Vec<Vec<u32>>,
}

impl Cloth {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height, coverage: vec![vec![0 as u32; width]; height] }
    }

    fn claim(&mut self, claim: &Claim) {
        for x in claim.left..claim.right {
            for y in claim.top..claim.bottom {
                self.coverage[y as usize][x as usize] += 1;
            }
        }
    }

    fn overlap_area(&self) -> usize {
        self.coverage.iter().map(|ref v| v.iter().filter(|&&count| count > 1).count()).sum()
    }

    fn has_no_overlap(&self, claim: &Claim) -> bool {
        for x in claim.left..claim.right {
            for y in claim.top..claim.bottom {
                if self.coverage[y as usize][x as usize] != 1 {
                    return false
                }
            }
        }
        return true
    }
}

#[derive(Debug)]
struct Claim {
    id: String,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = s.splitn(2, " @ ");
        match (parts.next(), parts.next()) {
            (Some(id), Some(coordinates)) => {
                let mut parts = coordinates.splitn(5, |c: char| !c.is_numeric()).filter_map(|s| s.parse::<u32>().ok());
                match (parts.next(), parts.next(), parts.next(), parts.next()) {
                    (Some(left), Some(top), Some(width), Some(height)) => {
                        Ok(Claim { id: id.to_string(), left, top, right: left + width, bottom: top + height })
                    }
                    _ => Err(format!("Invalid coordinates: {}", coordinates)),
                }
            }
            _ => Err(format!("Invalid claim: '{}'", s)),
        }
    }
}