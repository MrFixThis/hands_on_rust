pub mod jump_optim;
pub mod menu_optim;
pub mod score_optim;

/// The pendent state of an optimizer.
#[derive(Debug)]
pub struct Pendent;

/// The ready state of an optimizer.
#[derive(Debug)]
pub struct Ready;

/// A Reporter trait.
///
/// Abstract and concrete report of information related with the implementor.
pub trait Report {
    fn report(&self) -> String;
}

#[macro_export]
macro_rules! report {
    ( $target:ident ) => {
        println!(
            "+---------------------+\n\
            | Optimization Result |\n\
            +---------------------+\n\
            {}\
            ",
            $crate::problems::Report::report(&$target)
        )
    };
}
