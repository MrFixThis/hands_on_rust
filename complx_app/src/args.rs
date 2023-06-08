use std::{error::Error, str::FromStr};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ComplX App",
    version = "0.1.0",
    author = "Bryan Baron <MrFixThis>",
    rename_all = "kebab-case"
)]
pub struct TuiArgs {
    /// ComplX App's Sub-commands
    #[structopt(subcommand)]
    pub cmds: SubCmds,
}

#[derive(Debug, StructOpt)]
pub enum SubCmds {
    /// Discover the optimal meal menu based on a specified number of calories and a base menu.
    MenuOptimizer {
        /// Number of calories to meet with the optimal meal menu being generated.
        #[structopt(short, long, required = true)]
        target_calories: u32,

        /// Dishes from the base meal menu.
        ///
        /// They have to be spcified as follows: [-d | --dishes] dish/calories ...
        #[structopt(
            short = "d",
            long,
            required = true,
            parse(try_from_str = parse_key_value_pair),
            min_values = 2
        )]
        base_menu: Vec<(String, u32)>,
    },

    /// Determines the most prefered arbiters by an `N` number of teams and assigned to an `M (N/2)`
    /// number of soccer matches.
    ScoreOptimizer {
        /// Table of values with the rating given to the arbiters of the game.
        #[structopt(
            short = "p",
            long,
            required = true,
            parse(try_from_str = parse_row),
            min_values = 2
        )]
        preferences: Vec<Vec<isize>>,
    },

    /// Play around with a rabbit to find the minimum number of jumps it has to perform
    /// to take itself from a paint `A` to a point `B` in a farm field.
    JumpsOptimizer {
        /// Size of the farm field where the rabbit will be jumping.
        ///
        /// It is specified as follows: [-f | --field-size] `long/height`.
        #[structopt(
            short = "f",
            long,
            required = true,
            parse(try_from_str = parse_key_value_pair)
        )]
        field_size: (usize, usize),

        /// Length of the jump made by the rabbit.
        ///
        /// It is specified as follows: [-l | --jump-length] `lenght-x/lenght-y`.
        #[structopt(
            short = "l",
            long,
            required = true,
            parse(try_from_str = parse_key_value_pair)
        )]
        jump_length: (usize, usize),
        /// Coordinate of the starting point where the rabbit will start jumping.
        ///
        /// It is specified as follows: [-s | --start-point] `x/y`.
        #[structopt(
            short = "s",
            long,
            required = true,
            parse(try_from_str = parse_key_value_pair)
        )]
        start_point: (usize, usize),

        /// Coordinate of the target point where the rabbit will be arriving.
        ///
        /// It is specified as follows: [-t | --target-point] `x/y`.
        #[structopt(
            short = "t",
            long,
            required = true,
            parse(try_from_str = parse_key_value_pair)
        )]
        target_point: (usize, usize),
    },
}

fn parse_row<T>(s: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    s.split(',').map(|v| v.parse()).collect()
}

fn parse_key_value_pair<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: FromStr,
    T::Err: Error + 'static,
    U: FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('/')
        .ok_or_else(|| format!(r#"invalid key/value pair, no "/" found in {s}"#))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
