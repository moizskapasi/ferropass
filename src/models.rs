use serde::{Serialize, Deserialize};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    id: String,                  // 32-bit hash represented as a string
    username_or_email: String,   // Username or email for the account
    description: Option<String>, // Optional description
    password: String,            // Password for the account
}

impl Account {
    pub fn new(username_or_email: String, description: Option<String>, password: String) -> Self {
        let id = generate_id();
        Account {
            id,
            username_or_email,
            description,
            password,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_username_or_email(&self) -> &str {
        &self.username_or_email
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn set_username_or_email(&mut self, username_or_email: String) {
        self.username_or_email = username_or_email;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    accounts: Vec<Account>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            accounts: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn get_account_by_id(&self, id: &str) -> Option<&Account> {
        self.accounts.iter().find(|acc| acc.get_id() == id)
    }

    pub fn get_account_by_id_mut(&mut self, id: &str) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|acc| acc.get_id() == id)
    }

    pub fn remove_account(&mut self, id: &str) -> bool {
        let pos = self.accounts.iter().position(|acc| acc.get_id() == id);
        if let Some(pos) = pos {
            self.accounts.remove(pos);
            true
        } else {
            false
        }
    }
}

fn generate_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..u32::MAX);
    
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", timestamp, random_number));
    let result = hasher.result_str();
    
    result[..8].to_string()
}
