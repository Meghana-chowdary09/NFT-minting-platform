#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// NFT structure containing metadata
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub token_id: u64,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub metadata_uri: String,
    pub mint_time: u64,
}

// Platform statistics
#[contracttype]
#[derive(Clone)]
pub struct PlatformStats {
    pub total_minted: u64,
    pub total_owners: u64,
}

// Storage keys
const STATS: Symbol = symbol_short!("STATS");
const TOKEN_COUNT: Symbol = symbol_short!("T_COUNT");

// Mapping token_id to NFT
#[contracttype]
pub enum NFTBook {
    Token(u64)
}

#[contract]
pub struct NFTMintingContract;

#[contractimpl]
impl NFTMintingContract {
    
    /// Mint a new NFT with metadata
    pub fn mint_nft(
        env: Env,
        owner: Address,
        name: String,
        description: String,
        metadata_uri: String
    ) -> u64 {
        // Require owner authorization
        owner.require_auth();
        
        // Get current token count
        let mut token_count: u64 = env.storage().instance().get(&TOKEN_COUNT).unwrap_or(0);
        token_count += 1;
        
        // Get current timestamp
        let mint_time = env.ledger().timestamp();
        
        // Create NFT instance
        let nft = NFT {
            token_id: token_count,
            owner: owner.clone(),
            name: name.clone(),
            description,
            metadata_uri,
            mint_time,
        };
        
        // Update platform statistics
        let mut stats = Self::get_platform_stats(env.clone());
        stats.total_minted += 1;
        
        // Store NFT data
        env.storage().instance().set(&NFTBook::Token(token_count), &nft);
        env.storage().instance().set(&TOKEN_COUNT, &token_count);
        env.storage().instance().set(&STATS, &stats);
        
        // Extend storage TTL
        env.storage().instance().extend_ttl(100000, 100000);
        
        log!(&env, "NFT Minted: Token ID {}, Owner: {:?}, Name: {}", 
             token_count, owner, name);
        
        token_count
    }
    
    /// Get NFT details by token ID
    pub fn get_nft(env: Env, token_id: u64) -> NFT {
        let key = NFTBook::Token(token_id);
        
        env.storage().instance().get(&key).unwrap_or(NFT {
            token_id: 0,
            owner: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            name: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            metadata_uri: String::from_str(&env, ""),
            mint_time: 0,
        })
    }
    
    /// Get platform statistics
    pub fn get_platform_stats(env: Env) -> PlatformStats {
        env.storage().instance().get(&STATS).unwrap_or(PlatformStats {
            total_minted: 0,
            total_owners: 0,
        })
    }
    
    /// Transfer NFT ownership
    pub fn transfer_nft(env: Env, token_id: u64, new_owner: Address) {
        let mut nft = Self::get_nft(env.clone(), token_id);
        
        // Verify current owner is authorizing the transfer
        nft.owner.require_auth();
        
        // Ensure NFT exists
        if nft.token_id == 0 {
            log!(&env, "NFT does not exist");
            panic!("NFT does not exist");
        }
        
        // Update owner
        nft.owner = new_owner.clone();
        
        // Store updated NFT
        env.storage().instance().set(&NFTBook::Token(token_id), &nft);
        env.storage().instance().extend_ttl(100000, 100000);
        
        log!(&env, "NFT {} transferred to {:?}", token_id, new_owner);
    }
}