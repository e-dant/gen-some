mod names;
use clap::*;
use names::{LEFT, RIGHT};
use rand::{thread_rng, Rng};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Codename {
        #[arg(short, long, default_value = "1")]
        count: Option<u128>,
    },
    Ascii {
        #[arg(short, long, default_value = "1")]
        count: Option<u128>,
        #[arg(long)]
        case: Option<Case>,
    },
    Numeric {
        #[arg(short, long, default_value = "1")]
        count: Option<i128>,
        #[arg(long, default_value = "-170141183460469231731687303715884105728")]
        min: Option<i128>,
        #[arg(long, default_value = "170141183460469231731687303715884105726")]
        max: Option<i128>,
        #[arg(short, long)]
        decimal: bool,
    },
}

#[derive(Debug, ValueEnum, Copy, Clone, PartialEq, Eq)]
enum Case {
    Lower,
    Upper,
    Either,
}

fn main() {
    let mut rng = thread_rng();

    macro_rules! rand_pick_from {
        ($v:ident) => {
            $v[rng.gen_range(0..$v.len())]
        };
        ($($v:ident),*) => {
            // One result for each argument, all together as a tuple
            ( $(rand_pick_from!($v)), * )
        };
    }
    macro_rules! rand_ascii_lower {
        () => {
            std::char::from_u32(rng.gen_range(97..122)).unwrap()
        };
    }
    macro_rules! rand_ascii_upper {
        () => {
            std::char::from_u32(rng.gen_range(65..90)).unwrap()
        };
    }

    match Cli::parse().command {
        Command::Codename { count } => {
            let count = count.unwrap();
            for _ in 0..count {
                let (l, r) = rand_pick_from!(LEFT, RIGHT);
                println!("{l}-{r}");
            }
        }
        Command::Ascii { count, case } => {
            let count = count.unwrap();
            for _ in 0..count {
                let c = match case {
                    Some(Case::Lower) => rand_ascii_lower!(),
                    Some(Case::Upper) => rand_ascii_upper!(),
                    _ => {
                        if rng.gen_bool(0.5) {
                            rand_ascii_lower!()
                        } else {
                            rand_ascii_upper!()
                        }
                    }
                };
                println!("{c}");
            }
        }
        Command::Numeric {
            count,
            min,
            max,
            decimal,
        } => {
            let (count, min, max) = (count.unwrap(), min.unwrap(), max.unwrap());
            let max = if max <= min { min + 1 } else { max };
            if decimal {
                for _ in 0..count {
                    let l = rng.gen_range(min..max) - 1;
                    let r = &rng.gen_range(0_f64..1_f64).to_string()[2..];
                    println!("{l}.{r}");
                }
            } else {
                for _ in 0..count {
                    let v = rng.gen_range(min..max);
                    println!("{v}");
                }
            }
        }
    }
}
