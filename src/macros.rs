#[macro_export]
/// Returns early from the current function unless the condition holds.
macro_rules! return_unless {
    ($state:expr) => {
        if !$state {
            return;
        }
    };
}
