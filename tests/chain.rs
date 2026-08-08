use blockchain_rs::{storage, Blockchain, BlockchainError, Transaction};

#[test]
fn cadeia_recem_criada_e_valida() {
    let mut bc = Blockchain::new(2, 2_000, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_500, 1);
    assert!(bc.validate().is_ok());
}

#[test]
fn adulterar_dados_invalida_a_cadeia() {
    let mut bc = Blockchain::new(2, 2_000, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_500, 1);
    bc.blocks[1].transactions[0].amount = 999_999;
    assert_eq!(bc.validate(), Err(BlockchainError::InvalidHash { index: 1 }));
}

#[test]
fn dificuldade_sobe_quando_bloco_vem_rapido() {
    let mut bc = Blockchain::new(2, 2_000, 1_000);
    bc.add_block(vec![], 1_500, 1);          // 500ms < 2000ms → endurece
    assert_eq!(bc.blocks[1].difficulty, 3);
    assert!(bc.validate().is_ok());
}

#[test]
fn dificuldade_desce_quando_bloco_vem_lento() {
    let mut bc = Blockchain::new(3, 2_000, 1_000);
    bc.add_block(vec![], 6_000, 1);          // 5000ms > 2000ms → alivia
    assert_eq!(bc.blocks[1].difficulty, 2);
}

#[test]
fn mineracao_paralela_produz_bloco_valido() {
    let mut bc = Blockchain::new(2, 2_000, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_500, 4);
    assert!(bc.validate().is_ok());
}

#[test]
fn salvar_e_carregar_preserva_a_chain() {
    let mut bc = Blockchain::new(2, 2_000, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_500, 1);

    let path = std::env::temp_dir().join("blockchain_rs_roundtrip.json");
    storage::save(&bc, &path).expect("salvar deve funcionar");
    let loaded = storage::load(&path).expect("carregar deve funcionar");

    assert_eq!(loaded.blocks.len(), bc.blocks.len());
    assert_eq!(loaded.blocks[1].hash, bc.blocks[1].hash);
    let _ = std::fs::remove_file(&path);
}