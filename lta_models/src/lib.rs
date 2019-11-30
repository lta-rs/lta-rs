#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod taxi;
pub mod train;
pub mod traffic;