use std::error::Error;

use args::TuiArgs;

use crate::problems::{
    jump_optim::JumpOptimizer, menu_optim::MenuOptimizer, score_optim::ScoreOptimizer,
};

mod args;
mod problems;

fn main() -> Result<(), Box<dyn Error>> {
    let args: TuiArgs = structopt::StructOpt::from_args();

    match args.cmds {
        args::SubCmds::MenuOptimizer {
            target_calories,
            dishes,
        } => {
            todo!()
        }
        args::SubCmds::ScoreOptimizer {
            num_teams,
            num_arbiters,
            arbiters_prefs,
        } => {
            todo!()
        }
        args::SubCmds::JumpsOtimizer {
            field_size,
            jump_length,
            start_point,
            target_point,
        } => {
            todo!()
        }
    }
}
