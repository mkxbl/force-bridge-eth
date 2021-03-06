#![cfg_attr(not(feature = "std"), no_std)]

pub mod debug;

use ckb_env::traits::CkbChainInterface;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
    } else {
        extern crate alloc;
    }
}

pub fn verify<T: CkbChainInterface>(chain: T) -> i8 {
    let tx = chain.load_tx_hash();
    debug!("tx: {:?}", &tx);
    return 0;
}

#[cfg(test)]
mod tests {
    use ckb_env::mock::MockCKBChain;
    use super::verify;

    #[test]
    // #[should_panic(expected = "hello")]
    fn it_works() {
        let mock_chain = MockCKBChain::default();
        let return_code = verify(mock_chain);
        assert_eq!(return_code, 0);
    }
}
