use crate::hash::Hash;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: Hash,
    pub hash: Hash,
    pub nonce: u64,
    pub difficulty: usize,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: Hash,
        difficulty: usize,
        threads: usize,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: Hash::ZERO,
            nonce: 0,
            difficulty,
        };
        block.mine(threads);
        block
    }

    /// Todos os campos do bloco EXCETO o nonce, já convertidos em bytes.
    fn header_prefix(&self) -> Vec<u8> {
        let txs_json = serde_json::to_vec(&self.transactions)
            .expect("serialize cannot to fail");

        let mut buf = Vec::with_capacity(txs_json.len() + 128);
        buf.extend_from_slice(self.index.to_string().as_bytes());
        buf.push(b'|');
        buf.extend_from_slice(self.timestamp.to_string().as_bytes());
        buf.push(b'|');
        buf.extend_from_slice(&txs_json);
        buf.push(b'|');
        buf.extend_from_slice(self.previous_hash.as_bytes());
        buf.push(b'|');
        buf.extend_from_slice(self.difficulty.to_string().as_bytes());
        buf.push(b'|');
        buf
    }

    fn mine(&mut self, threads: usize) {
        let threads = threads.max(1);
        let prefix = self.header_prefix();
        let difficulty = self.difficulty;

        // Caminho sequencial: determinístico, usado nos testes.
        if threads == 1 {
            let (found_nonce, found_hash) = search(&prefix, 0, 1, difficulty, None)
                .expect("busca sem sinal de parada só retorna quando encontra");
            self.nonce = found_nonce;
            self.hash = found_hash;
            return;
        }

        let found = AtomicBool::new(false);
        let winner: Mutex<Option<(u64, Hash)>> = Mutex::new(None);

        std::thread::scope(|scope| {
            for t in 0..threads {
                let (found, winner, prefix) = (&found, &winner, &prefix);
                scope.spawn(move || {
                    if let Some(result) =
                        search(prefix, t as u64, threads as u64, difficulty, Some(found))
                    {
                        *winner.lock().expect("mutex envenenado") = Some(result);
                        found.store(true, Ordering::Relaxed);
                    }
                });
            }
        });

        let (found_nonce, found_hash) = winner
            .into_inner()
            .expect("mutex envenenado")
            .expect("alguma thread precisa ter encontrado um nonce válido");
        self.nonce = found_nonce;
        self.hash = found_hash;
    }

    pub fn has_valid_hash(&self) -> bool {
        let mut buf = self.header_prefix();
        buf.extend_from_slice(self.nonce.to_string().as_bytes());
        let digest: [u8; 32] = Sha256::digest(&buf).into();
        self.hash == Hash::from_bytes(digest)
    }

    pub fn meets_difficulty(&self) -> bool {
        self.hash.has_leading_zeros(self.difficulty)
    }
}

/// Busca por força bruta. Retorna None se abortou por sinal de outra thread.
fn search(
    prefix: &[u8],
    start: u64,
    stride: u64,
    difficulty: usize,
    stop: Option<&AtomicBool>,
) -> Option<(u64, Hash)> {
    let mut hasher = Sha256::new();
    let mut buf = Vec::with_capacity(prefix.len() + 24);
    let mut digits = [0u8; 20];
    let mut nonce = start;
    let mut since_check: u32 = 0;

    loop {
        since_check += 1;
        if since_check >= 1024 {
            since_check = 0;
            if let Some(flag) = stop {
                if flag.load(Ordering::Relaxed) {
                    return None;
                }
            }
        }

        buf.clear();
        buf.extend_from_slice(prefix);
        buf.extend_from_slice(write_u64(&mut digits, nonce));

        hasher.update(&buf);
        let digest: [u8; 32] = hasher.finalize_reset().into();
        let hash = Hash::from_bytes(digest);

        if hash.has_leading_zeros(difficulty) {
            return Some((nonce, hash));
        }
        nonce = nonce.wrapping_add(stride);
    }
}



/// Escreve um u64 como texto decimal num buffer da stack. Zero alocação.
fn write_u64(scratch: &mut [u8; 20], mut n: u64) -> &[u8] {
    if n == 0 {
        scratch[19] = b'0';
        return &scratch[19..];
    }
    let mut i = 20;
    while n > 0 {
        i -= 1;
        scratch[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    &scratch[i..]
}