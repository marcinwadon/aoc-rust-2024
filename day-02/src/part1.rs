use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, line_ending, newline, space1},
    multi::separated_list1,
    IResult,
};
use tracing::instrument;

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let result = reports
        .iter()
        .inspect(|report| {
            dbg!(report, check_safety(report));
        })
        .map(check_safety)
        .filter(|safety| safety == &Safety::Safe)
        .count();

    Ok(result.to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Safety {
    let mut direction: Option<Direction> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Safety::Unsafe;
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    } else {
                        direction = Some(Direction::Decreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff) {
                        return Safety::Unsafe;
                    } else {
                        continue;
                    }
                }
                Some(Direction::Decreasing) => {
                    return Safety::Unsafe;
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    } else {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Safety::Unsafe;
            }
            _ => {
                panic!("should never have a non -1,1,0 number");
            }
        }
    }

    Safety::Safe
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
