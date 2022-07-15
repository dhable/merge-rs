use merge_rs::mergeable::Mergable;
use merge_rs::Merge;

fn add(lhs: &u32, rhs: &u32) -> u32 {
    lhs + rhs
}

fn max(lhs: &u32, rhs: &u32) -> u32 {
    if lhs < rhs {
        *rhs
    } else {
        *lhs
    }
}

#[derive(Debug, Clone)]
struct PrefixOpts {
    #[allow(dead_code)]
    v1: String,
    #[allow(dead_code)]
    v2: String,
}

impl PrefixOpts {
    fn new(v1: String, v2: String) -> PrefixOpts {
        Self { v1, v2 }
    }
}

impl Merge for PrefixOpts {
    fn merge(&self, rhs: &Self) -> Self {
        // does a right bias merge
        rhs.clone()
    }
}

#[derive(Debug)]
struct Opts {
    replicas: Mergable<u32>,
    max_timeout: Mergable<u32>,
    prefixes: Option<PrefixOpts>,
}

impl Opts {
    fn new(replicas: u32, max_timeout: u32, prefixes: Option<PrefixOpts>) -> Opts {
        Self {
            replicas: Mergable::new(replicas, add),
            max_timeout: Mergable::new(max_timeout, max),
            prefixes,
        }
    }
}

impl Merge for Opts {
    fn merge(&self, rhs: &Self) -> Self {
        Self {
            replicas: self.replicas.merge(&rhs.replicas),
            max_timeout: self.max_timeout.merge(&rhs.max_timeout),
            prefixes: self.prefixes.merge(&rhs.prefixes),
        }
    }
}

fn main() {
    let env_opts = Opts::new(
        3,
        30_000,
        Some(PrefixOpts::new("env1".to_string(), "env2".to_string())),
    );

    let mut legacy_opts = Opts::new(
        9,
        90_000,
        Some(PrefixOpts::new("new-1".to_string(), "new-2".to_string())),
    );
    legacy_opts.replicas -= 6;

    let new_opts = Opts::new(5, 15_000, None);

    let resolved_opts = env_opts.merge(&legacy_opts).merge(&new_opts);
    println!("{:#?}", resolved_opts);
}
