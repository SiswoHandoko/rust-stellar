#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }

    // Fungsi untuk menyimpan transaksi sederhana
    pub fn record_transaction(env: Env, from: Address, to: Address, amount: i128) {
        // Simpan transaksi dalam log untuk keperluan catatan
        env.events()
            .publish((symbol_short!("Trans"), from.clone(), to.clone()), amount);

        // stellar contract invoke `
        // --id CBVTB44QFRTR5STFE5QWXVUJ3DZF5YXQSU7PYU5Q5UU6E2N74IO5J222 `
        // --source alice `
        // --network testnet `
        // --send=yes `
        // -- `
        // record_transaction `
        // --from GDFW2TLM3SLIOGOIVTHTYCFKQJV6KTE74QYADDOAI2KOUX7EFDAWO3UH `
        // --to GADCGFVPS3KXTFE3MSWG57PWASGGHBQIZVWDKVGHJ3P7KDVAFYH6RFYH `
        // --amount 1000
    }
}

mod test;
