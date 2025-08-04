use validblock_core::wallet::mock::MockWallet;

#[test]
fn test_mock_wallet_sign() {
    let w = MockWallet;
    let msg = b"hello";
    let sig = w.sign(msg).expect("sig");
    assert!(w.verify(msg, &sig).unwrap());
}