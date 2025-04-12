use crate::clipboard::copy_to_clipboard;
use crate::encryption::{encrypt_and_save_database, load_and_decrypt_database};
use crate::models::{Account, Database};
use crate::password::{generate_random_password, is_password_valid};

use std::io::{self, Write};
use std::path::PathBuf;
use rpassword::read_password;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

pub struct CLI {
    current_database_path: Option<PathBuf>,
    current_database: Option<Database>,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            current_database_path: None,
            current_database: None,
        }
    }
    
    pub fn clear_screen() -> Result<(), String> {
        if let Err(e) = execute!(io::stdout(), Clear(ClearType::All)) {
            return Err(format!("Failed to clear screen: {}", e));
        }
        Ok(())
    }
    
    pub fn prompt_input(prompt: &str) -> Result<String, String> {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
        
        Ok(input.trim().to_string())
    }
    
    pub fn prompt_password(prompt: &str) -> Result<String, String> {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
        
        read_password().map_err(|e| format!("Failed to read password: {}", e))
    }
    
    pub fn run(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        
        loop {
            println!("=== FP Password Manager ===");
            println!("1. Create a new password database");
            println!("2. Open an existing password database");
            println!("3. Exit");
            
            let choice = Self::prompt_input("Enter your choice (1-3): ")?;
            
            match choice.as_str() {
                "1" => self.create_new_database()?,
                "2" => self.open_existing_database()?,
                "3" => break,
                _ => {
                    println!("Invalid choice, please try again.");
                    continue;
                }
            }
            
            if self.current_database.is_some() {
                self.database_menu()?;
            }
        }
        
        Ok(())
    }
    
    fn create_new_database(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Create New Database ===");
        
        let db_name = Self::prompt_input("Enter database name (without extension): ")?;
        let mut filepath = PathBuf::from(&db_name);
        filepath.set_extension("fp");
        
        if filepath.exists() {
            println!("A database with this name already exists. Please choose a different name.");
            return Ok(());
        }
        
        let passkey = self.prompt_for_valid_passkey()?;
        
        let database = Database::new();
        
        encrypt_and_save_database(&database, &filepath, &passkey)?;
        
        println!("Database created successfully!");
        
        self.current_database_path = Some(filepath);
        self.current_database = Some(database);
        
        Ok(())
    }
    
    fn open_existing_database(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Open Existing Database ===");
        
        let filepath_str = Self::prompt_input("Enter absolute path to database file (.fp): ")?;
        let filepath = PathBuf::from(filepath_str);
        
        if !filepath.exists() {
            println!("File not found. Please check the path and try again.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        let passkey = Self::prompt_password("Enter database passkey: ")?;
        
        if passkey.is_empty() {
            println!("Passkey cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        match load_and_decrypt_database(&filepath, &passkey) {
            Ok(database) => {
                println!("Database loaded successfully!");
                self.current_database_path = Some(filepath);
                self.current_database = Some(database);
            },
            Err(e) => {
                println!("Failed to open database: {}", e);
                Self::prompt_input("Press Enter to continue...")?;
            }
        }
        
        Ok(())
    }
    
    fn database_menu(&mut self) -> Result<(), String> {
        loop {
            Self::clear_screen()?;
            
            println!("=== Database Menu ===");
            println!("Database: {:?}", self.current_database_path.as_ref().unwrap());
            println!("1. List accounts");
            println!("2. View/Edit account");
            println!("3. Add new account");
            println!("4. Delete account");
            println!("5. Return to main menu");
            
            let choice = Self::prompt_input("Enter your choice (1-5): ")?;
            
            match choice.as_str() {
                "1" => self.list_accounts()?,
                "2" => self.view_edit_account()?,
                "3" => self.add_account()?,
                "4" => self.delete_account()?,
                "5" => break,
                _ => {
                    println!("Invalid choice, please try again.");
                    continue;
                }
            }
            
        }
        
        Ok(())
    }
    
    fn list_accounts(&self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Account List ===");
        
        if let Some(db) = &self.current_database {
            let accounts = db.get_accounts();
            
            if accounts.is_empty() {
                println!("No accounts found in the database.");
            } else {
                println!("{:<10} {:<30} {:<20}", "ID", "Username/Email", "Description");
                println!("{:-<60}", "");
                
                for account in accounts {
                    let desc = account.get_description()
                        .as_ref()
                        .map_or("", |s| s.as_str());
                    
                    println!("{:<10} {:<30} {:<20}", 
                        account.get_id(),
                        account.get_username_or_email(),
                        desc
                    );
                }
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn view_edit_account(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== View/Edit Account ===");
        
        if let Some(db) = &self.current_database {
            let accounts = db.get_accounts();
            
            if accounts.is_empty() {
                println!("No accounts found in the database.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            } else {
                println!("{:<10} {:<30} {:<20}", "ID", "Username/Email", "Description");
                println!("{:-<60}", "");
                
                for account in accounts {
                    let desc = account.get_description()
                        .as_ref()
                        .map_or("", |s| s.as_str());
                    
                    println!("{:<10} {:<30} {:<20}", 
                        account.get_id(),
                        account.get_username_or_email(),
                        desc
                    );
                }
                println!();
            }
        } else {
            println!("No database loaded.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        let account_id = Self::prompt_input("Enter account ID: ")?;
        
        if let Some(db) = &self.current_database {
            if let Some(_account) = db.get_account_by_id(&account_id) {
                self.account_menu(&account_id)?;
            } else {
                println!("Account not found.");
                Self::prompt_input("Press Enter to continue...")?;
            }
        } else {
            println!("No database loaded.");
            Self::prompt_input("Press Enter to continue...")?;
        }
        
        Ok(())
    }
    
    fn account_menu(&mut self, account_id: &str) -> Result<(), String> {
        loop {
            Self::clear_screen()?;
            
            let account = if let Some(db) = &self.current_database {
                if let Some(acc) = db.get_account_by_id(account_id) {
                    acc.clone()
                } else {
                    println!("Account not found.");
                    return Ok(());
                }
            } else {
                println!("No database loaded.");
                return Ok(());
            };
            
            println!("=== Account Details ===");
            println!("ID: {}", account.get_id());
            println!("Username/Email: {}", account.get_username_or_email());
            println!("Description: {}", account.get_description().as_ref().map_or("", |s| s.as_str()));
            println!("Password: [HIDDEN]");
            println!();
            println!("1. Edit account information");
            println!("2. Copy password to clipboard");
            println!("3. Generate new password");
            println!("4. Return to database menu");
            
            let choice = Self::prompt_input("Enter your choice (1-4): ")?;
            
            match choice.as_str() {
                "1" => self.edit_account(account_id)?,
                "2" => self.copy_password(account_id)?,
                "3" => self.generate_new_password(account_id)?,
                "4" => break,
                _ => {
                    println!("Invalid choice, please try again.");
                    continue;
                }
            }
        }
        
        Ok(())
    }
    
    fn edit_account(&mut self, account_id: &str) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Edit Account ===");
        
        let passkey = Self::prompt_password("Enter database passkey: ")?;
        
        if passkey.is_empty() {
            println!("Passkey cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        if let Some(path) = &self.current_database_path {
            if load_and_decrypt_database(path, &passkey).is_err() {
                println!("Invalid passkey. Changes not made.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            }
            
            if let Some(db) = &mut self.current_database {
                if let Some(account) = db.get_account_by_id_mut(account_id) {
                    println!("Current Username/Email: {}", account.get_username_or_email());
                    let new_username = Self::prompt_input("Enter new Username/Email (leave empty to keep current): ")?;
                    
                    if !new_username.is_empty() {
                        account.set_username_or_email(new_username);
                    }
                    
                    let current_desc = account.get_description().as_ref().map_or("", |s| s.as_str());
                    println!("Current Description: {}", current_desc);
                    let new_desc = Self::prompt_input("Enter new Description (leave empty to keep current): ")?;
                    
                    if !new_desc.is_empty() {
                        account.set_description(Some(new_desc));
                    } else if new_desc.is_empty() && !current_desc.is_empty() {
                        let keep_desc = Self::prompt_input("Do you want to keep the current description? (y/n): ")?;
                        if keep_desc.to_lowercase() == "n" {
                            account.set_description(None);
                        }
                    }
                    
                    println!("Edit password? (y/n): ");
                    let edit_password = Self::prompt_input("")?;
                    
                    if edit_password.to_lowercase() == "y" {
                        let password_action = Self::prompt_input("Do you want to (1) enter a new password or (2) generate a random one? (1/2): ")?;
                        
                        if password_action == "1" {
                            let mut valid_password = false;
                            let mut new_password = String::new();
                            
                            while !valid_password {
                                new_password = Self::prompt_password("Enter new password: ")?;
                                
                                if is_password_valid(&new_password) {
                                    valid_password = true;
                                } else {
                                    println!("Password must be at least 15 characters, contain at least one uppercase letter, one lowercase letter, one number, and one special character.");
                                }
                            }
                            
                            account.set_password(new_password);
                            println!("Password updated successfully!");
                        } else if password_action == "2" {
                            let new_password = generate_random_password();
                            account.set_password(new_password.clone());
                            println!("Generated password: {}", new_password);
                            println!("Password updated successfully!");
                        } else {
                            println!("Invalid choice, password not updated.");
                        }
                    }
                    
                    println!("Account updated successfully!");
                    
                    encrypt_and_save_database(db, path, &passkey)?;
                    println!("Changes saved successfully!");
                } else {
                    println!("Account not found.");
                }
            } else {
                println!("No database loaded.");
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn copy_password(&self, account_id: &str) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Copy Password ===");
        
        let passkey = Self::prompt_password("Enter database passkey: ")?;
        
        if passkey.is_empty() {
            println!("Passkey cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        if let Some(path) = &self.current_database_path {
            if load_and_decrypt_database(path, &passkey).is_err() {
                println!("Invalid passkey. Password not copied.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            }
            
            if let Some(db) = &self.current_database {
                if let Some(account) = db.get_account_by_id(account_id) {
                    copy_to_clipboard(account.get_password())?;
                    println!("Password copied to clipboard!");
                } else {
                    println!("Account not found.");
                }
            } else {
                println!("No database loaded.");
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn generate_new_password(&mut self, account_id: &str) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Generate New Password ===");
        
        let passkey = Self::prompt_password("Enter database passkey: ")?;
        
        if passkey.is_empty() {
            println!("Passkey cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        if let Some(path) = &self.current_database_path {
            if load_and_decrypt_database(path, &passkey).is_err() {
                println!("Invalid passkey. Password not generated.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            }
            
            if let Some(db) = &mut self.current_database {
                if let Some(account) = db.get_account_by_id_mut(account_id) {
                    let new_password = generate_random_password();
                    
                    println!("Generated password: {}", new_password);
                    let confirm = Self::prompt_input("Do you want to set this as the new password? (y/n): ")?;
                    
                    if confirm.to_lowercase() == "y" {
                        account.set_password(new_password);
                        println!("Password updated successfully!");
                        
                        encrypt_and_save_database(db, path, &passkey)?;
                        println!("Changes saved successfully!");
                    } else {
                        println!("Password not updated.");
                    }
                } else {
                    println!("Account not found.");
                }
            } else {
                println!("No database loaded.");
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn add_account(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Add New Account ===");
        
        let username = Self::prompt_input("Enter Username/Email: ")?;
        
        if username.is_empty() {
            println!("Username/Email cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        let description = Self::prompt_input("Enter Description (optional): ")?;
        let description = if description.is_empty() { None } else { Some(description) };
        
        let password_choice = Self::prompt_input("Do you want to (1) enter your own password or (2) generate a random one? (1/2): ")?;
        
        let password = if password_choice == "1" {
            let mut valid_password = false;
            let mut pwd = String::new();
            
            while !valid_password {
                pwd = Self::prompt_password("Enter password: ")?;
                
                if is_password_valid(&pwd) {
                    valid_password = true;
                } else {
                    println!("Password must be at least 15 characters, contain at least one uppercase letter, one lowercase letter, one number, and one special character.");
                }
            }
            
            pwd
        } else if password_choice == "2" {
            let pwd = generate_random_password();
            println!("Generated password: {}", pwd);
            pwd
        } else {
            println!("Invalid choice. Using a generated password.");
            let pwd = generate_random_password();
            println!("Generated password: {}", pwd);
            pwd
        };
        
        if let Some(db) = &mut self.current_database {
            if let Some(path) = &self.current_database_path {
                let passkey = Self::prompt_password("Enter database passkey to save changes: ")?;
                
                if passkey.is_empty() {
                    println!("Passkey cannot be empty.");
                    Self::prompt_input("Press Enter to continue...")?;
                    return Ok(());
                }
                
                if load_and_decrypt_database(path, &passkey).is_err() {
                    println!("Invalid passkey. Account not created.");
                    Self::prompt_input("Press Enter to continue...")?;
                    return Ok(());
                }
                
                let account = Account::new(username, description, password);
                db.add_account(account);
                
                encrypt_and_save_database(db, path, &passkey)?;
                println!("Account added successfully!");
                println!("Changes saved successfully!");
            } else {
                println!("No database path found.");
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn delete_account(&mut self) -> Result<(), String> {
        Self::clear_screen()?;
        println!("=== Delete Account ===");
        
        if let Some(db) = &self.current_database {
            let accounts = db.get_accounts();
            
            if accounts.is_empty() {
                println!("No accounts found in the database.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            } else {
                println!("{:<10} {:<30} {:<20}", "ID", "Username/Email", "Description");
                println!("{:-<60}", "");
                
                for account in accounts {
                    let desc = account.get_description()
                        .as_ref()
                        .map_or("", |s| s.as_str());
                    
                    println!("{:<10} {:<30} {:<20}", 
                        account.get_id(),
                        account.get_username_or_email(),
                        desc
                    );
                }
                println!();
            }
        } else {
            println!("No database loaded.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        let account_id = Self::prompt_input("Enter account ID to delete: ")?;
        
        if let Some(db) = &self.current_database {
            if db.get_account_by_id(&account_id).is_none() {
                println!("Account not found.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            }
        } else {
            println!("No database loaded.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        let passkey = Self::prompt_password("Enter database passkey: ")?;
        
        if passkey.is_empty() {
            println!("Passkey cannot be empty.");
            Self::prompt_input("Press Enter to continue...")?;
            return Ok(());
        }
        
        if let Some(path) = &self.current_database_path {
            if load_and_decrypt_database(path, &passkey).is_err() {
                println!("Invalid passkey. Deletion cancelled.");
                Self::prompt_input("Press Enter to continue...")?;
                return Ok(());
            }
            
            let confirm = Self::prompt_input("Are you sure you want to delete this account? (y/n): ")?;
            
            if confirm.to_lowercase() == "y" {
                if let Some(db) = &mut self.current_database {
                    if db.remove_account(&account_id) {
                        println!("Account deleted successfully!");
                        
                        encrypt_and_save_database(db, path, &passkey)?;
                        println!("Changes saved successfully!");
                    } else {
                        println!("Account not found.");
                    }
                } else {
                    println!("No database loaded.");
                }
            } else {
                println!("Deletion cancelled.");
            }
        } else {
            println!("No database loaded.");
        }
        
        Self::prompt_input("Press Enter to continue...")?;
        Ok(())
    }
    
    fn prompt_for_valid_passkey(&self) -> Result<String, String> {
        loop {
            let passkey = Self::prompt_password("Enter database passkey (min. 15 chars, must include uppercase, lowercase, number, and special character): ")?;
            
            if passkey.is_empty() {
                println!("Passkey cannot be empty.");
                continue;
            }
            
            if is_password_valid(&passkey) {
                let confirm_passkey = Self::prompt_password("Confirm passkey: ")?;
                
                if confirm_passkey.is_empty() {
                    println!("Confirmation passkey cannot be empty.");
                    continue;
                }
                
                if passkey == confirm_passkey {
                    return Ok(passkey);
                } else {
                    println!("Passkeys do not match. Please try again.");
                }
            } else {
                println!("Invalid passkey. It must be at least 15 characters, and contain at least one uppercase letter, one lowercase letter, one number, and one special character.");
            }
        }
    }
}