use blockchain_rs::{Blockchain, BlockchainError, Transaction, storage};

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

#[test]
fn salvar_e_carregar_preserva_a_chain() {
    let mut bc = Blockchain::new(3, 10, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_002);

    let path = std::env::temp_dir().join("blockchain_rs_roundtrip.json");
    storage::save(&bc, &path).expect("Saving should succeed");
    let loaded = storage::load(&path).expect("Loading should succeed");

    assert_eq!(loaded.blocks.len(), bc.blocks.len());
    assert_eq!(loaded.blocks[1].hash, bc.blocks[1].hash);

    let _ = std::fs::remove_file(&path);  // cleanup
    }

#[test]
fn carregar_arquivo_adulterado_e_rejeitado() {
    let mut bc = Blockchain::new(2, 10, 1_000);
    bc.add_block(vec![Transaction::new("Alice", "Bob", 10)], 1_002);

    let path = std::env::temp_dir().join("blockchain_rs_tampered.json");
    storage::save(&bc, &path).unwrap();

    // Adulterando o arquivo 
    let json = std::fs::read_to_string(&path).unwrap();
    let tampered = json.replace("\"amount\": 10", "\"amount\": 999999");
    std::fs::write(&path, tampered).unwrap();

    assert!(storage::load(&path).is_err());

    let _ = std::fs::remove_file(&path);
}
