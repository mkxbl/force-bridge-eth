#![feature(async_closure)]
pub mod header_relay;
pub mod dapp;
pub mod transfer;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
