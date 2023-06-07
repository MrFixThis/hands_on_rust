use std::marker::PhantomData;

use super::{Report, Pendent, Ready};

#[derive(Debug)]
pub struct ScoreOptimizer<State = Pendent> {
    _marker: PhantomData<State>
}

impl Report for ScoreOptimizer {
    fn report(&self) -> String {
        todo!()
    }
}

impl From<ScoreOptimizer<Pendent>> for ScoreOptimizer<Ready> {
    fn from(so: ScoreOptimizer<Pendent>) -> Self {
        todo!()
    }
}
