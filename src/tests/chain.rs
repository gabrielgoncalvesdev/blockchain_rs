use blockchain_rs::{Blockchain, BlockchainError};

#[test]
fn cadeia_recem_criada_e_valida() {
    let mut bc = Blockchain::new(3);
    bc.add_block("Alice paga Bob 10 BTC".into());
    assert!(bc.validate().is_ok());
}

#[test]
fn adulterar_dados_invalida_a_cadeia() {
    let mut bc = Blockchain::new(3);
    bc.add_block("Alice paga Bob 10 BTC".into());
    bc.blocks[1].data = "Alice paga Bob 1000 BTC".into();  // atacante
    assert_eq!(bc.validate(), Err(BlockchainError::InvalidHash { index: 1 }));
}