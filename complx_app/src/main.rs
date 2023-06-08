#![feature(iter_intersperse)]

use std::error::Error;

use args::TuiArgs;

use crate::problems::{
    jump_optim::JumpOptimizer, menu_optim::MenuOptimizer, score_optim::ScoreOptimizer,
};

mod args;
mod problems;

fn main() -> Result<(), Box<dyn Error>> {
    let args = <TuiArgs as structopt::StructOpt>::from_args();
    match args.cmds {
        args::SubCmds::MenuOptimizer {
            target_calories,
            base_menu,
        } => {
            let mor = MenuOptimizer::new().find_optimal_menu(target_calories, base_menu);
            report!(mor);
        }
        args::SubCmds::ScoreOptimizer { preferences } => {
            let sor = ScoreOptimizer::build(preferences)?.find_optimal_assigment();
            report!(sor);
        }
        args::SubCmds::JumpsOptimizer {
            field_size,
            jump_length,
            start_point,
            target_point,
        } => {
            let jor = JumpOptimizer::new(field_size, jump_length)
                .find_min_jumps(start_point, target_point)?;
            report!(jor);
        }
    }

    Ok(())
}
