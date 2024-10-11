mod utils;

use std::convert::TryInto;
use wasm_bindgen::prelude::*;

use drillx_2::equix;
use drillx_2;
use log::Level;
use console_log;
use web_sys::js_sys::Date;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct MiningResult {
    pub best_nonce: u64,
    pub best_difficulty: u32,
    pub total_hashes: u64,
}

#[wasm_bindgen]
#[target_feature(enable = "simd128")]
pub fn start_mining(challenge: &[u8], nonce_range_start: u64, nonce_range_end: u64, cutoff: f64) -> MiningResult {
    console_log::init_with_level(Level::Debug).expect("error initializing log");
    //log::debug!("Inside the start_mining function...");
    //log::debug!("Setting everything up...");
    let mut memory = equix::SolverMemory::new();
    //log::debug!("Step 1...");
    let mut best_nonce = nonce_range_start;
    //log::debug!("Step 2...");
    let mut best_difficulty = 0;
    //log::debug!("Step 3...");
    let mut total_hashes: u64 = 0;
    //log::debug!("Step 4...");

    let hash_timer = Date::now();
    //log::debug!("Step 5...");
    let mut nonce = nonce_range_start;

    //log::debug!("Fixing up challenge...");
    // Create hash for current nonce
    let challenge_fixed: &[u8; 32] = challenge.try_into().expect("Challenge should be 32 bytes");
    //log::debug!("Starting nonce loop...");

    while nonce < nonce_range_end {
        // log::debug!("Looping nonce...");
        for hx in drillx_2::get_hashes_with_memory(&mut memory, challenge_fixed, &nonce.to_le_bytes()) {
            total_hashes += 1;
            let difficulty = hx.difficulty();

            // Update best nonce if better difficulty is found
            if difficulty > best_difficulty {
                best_nonce = nonce;
                best_difficulty = difficulty;
            }
        }

        //let elapsed = performance.now() - hash_timer;
        let elapsed = Date::now() - hash_timer;

        // Check if the time cutoff has been reached
        if elapsed / 1000.0 >= cutoff {
            break;
        }

        // Increment nonce
        nonce += 1;
    }

    // Return the best nonce and the total hashes calculated
    MiningResult {
        best_nonce,
        best_difficulty,
        total_hashes,
    }
}
