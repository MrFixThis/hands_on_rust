pub mod jump_optim;
pub mod menu_optim;
pub mod score_optim;

/// A Reporter trait.
///
/// Abstract and concrete report of information related with the implementor.
pub trait Report {
    fn report(&self);
}
