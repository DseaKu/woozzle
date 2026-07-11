#[macro_export]
/// Procceed only if the state is true, else return
macro_rules! guard_update {
    ($state:expr) => {
        if !$state {
            return;
        }
    };
}
