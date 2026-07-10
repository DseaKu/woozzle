#[macro_export]
macro_rules! guard_update {
    ($state:expr) => {
        if !$state {
            return;
        }
    };
}
