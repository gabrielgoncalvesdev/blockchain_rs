use blockchain_rs::{Blockchain, BlockchainError, Transaction};

#[test]
fn cadeia_recem_criada_e_valida() {
    let mut bc = Blockchain::new(3, 1_690_000_000);
    bc.add_block("Alice paga Bob 10 BTC".into(), 1_690_000_000);
    assert!(bc.validate().is_ok());
}

#[test]
fn adulterar_dados_invalida_a_cadeia() {
    let mut bc = Blockchain::new(3, 1_690_000_000);
bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_002);
    bc.blocks[1].transactions[0].amount = 999_999;  // atacante
    assert_eq!(bc.validate(), Err(BlockchainError::InvalidHash { index: 1 }));
}

#[test]
fn dificuldade_sobe_quando_bloco_vem_rapido() {
    let mut bc = Blockchain::new(2, 10, 1_000);
    bc.add_block("txt".into(), 1_005);
    assert_eq!(bc.blocks[1].difficulty, 3);
    assert!(bc.validate().is_ok());
}

#[test]
fn dificuldade_desce_quando_bloco_vem_lento() {
    let mut bc = Blockchain::new(3, 10, 1_000);
    bc.add_block("tx".into(), 1_050);        // 50s > alvo de 10s → alivia
    assert_eq!(bc.blocks[1].difficulty, 2);
}
