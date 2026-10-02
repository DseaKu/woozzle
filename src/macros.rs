#[macro_export]
/// Procceed only if the state is true, else return
macro_rules! return_unless {
    ($state:expr) => {
        if !$state {
            return;
        }
    };
}
