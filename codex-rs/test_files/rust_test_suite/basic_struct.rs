// Rust Test Suite - Basic Structures and Implementations
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt::{Display, Debug};

// Constants
const DEFAULT_ROLE: &str = "user";
const MAX_LOGIN_ATTEMPTS: u32 = 3;

// Type aliases
type UserId = u64;
type UserRole = String;

// Enums
#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Pending,
    Suspended,
}

// Traits
pub trait Identifiable {
    fn get_id(&self) -> UserId;
}

pub trait Validatable {
    fn validate(&self) -> Result<(), Vec<String>>;
}

// Generic trait
pub trait Repository<T> {
    fn find_by_id(&self, id: UserId) -> Option<T>;
    fn save(&mut self, entity: T) -> Result<(), String>;
    fn delete(&mut self, id: UserId) -> bool;
}

// Structs
#[derive(Debug, Clone)]
pub struct BaseEntity {
    pub id: UserId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub base: BaseEntity,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub status: UserStatus,
    pub role: UserRole,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, String>,
}

// Implementation blocks
impl BaseEntity {
    pub fn new(id: UserId) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

impl User {
    // Constructor
    pub fn new(id: UserId, name: String, email: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if !Self::is_valid_email(&email) {
            return Err("Invalid email format".to_string());
        }
        
        Ok(Self {
            base: BaseEntity::new(id),
            name,
            email,
            is_active: true,
            status: UserStatus::Active,
            role: DEFAULT_ROLE.to_string(),
            permissions: Vec::new(),
            metadata: HashMap::new(),
        })
    }
    
    // Static method
    pub fn create_guest() -> Self {
        Self {
            base: BaseEntity::new(0),
            name: "Guest".to_string(),
            email: "guest@example.com".to_string(),
            is_active: false,
            status: UserStatus::Inactive,
            role: "guest".to_string(),
            permissions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    // Instance methods
    pub fn update_email(&mut self, new_email: String) -> Result<(), String> {
        if !Self::is_valid_email(&new_email) {
            return Err("Invalid email format".to_string());
        }
        
        self.email = new_email;
        self.base.touch();
        Ok(())
    }
    
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
            self.base.touch();
        }
    }
    
    pub fn remove_permission(&mut self, permission: &str) -> bool {
        if let Some(pos) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(pos);
            self.base.touch();
            true
        } else {
            false
        }
    }
    
    pub fn get_display_name(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }
    
    // Private method
    fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
    
    // Generic method
    pub fn get_metadata<T>(&self, key: &str) -> Option<T> 
    where 
        T: std::str::FromStr,
    {
        self.metadata.get(key)?.parse().ok()
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.base.touch();
    }
}

// Trait implementations
impl Identifiable for User {
    fn get_id(&self) -> UserId {
        self.base.id
    }
}

impl Validatable for User {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.name.is_empty() {
            errors.push("Name is required".to_string());
        }
        
        if !Self::is_valid_email(&self.email) {
            errors.push("Invalid email format".to_string());
        }
        
        if self.role.is_empty() {
            errors.push("Role is required".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ id: {}, name: {}, email: {}, active: {} }}", 
               self.base.id, self.name, self.email, self.is_active)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.base.id == other.base.id
    }
}

// Generic struct
#[derive(Debug)]
pub struct UserRepository<T> {
    users: Arc<Mutex<HashMap<UserId, T>>>,
}

impl<T> UserRepository<T> 
where 
    T: Clone + Identifiable,
{
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn add_user(&self, user: T) -> Result<(), String> {
        let mut users = self.users.lock().map_err(|_| "Lock error")?;
        users.insert(user.get_id(), user);
        Ok(())
    }
    
    pub fn get_user(&self, id: UserId) -> Option<T> {
        let users = self.users.lock().ok()?;
        users.get(&id).cloned()
    }
}

// Functions
pub fn process_users(users: Vec<&User>) -> Vec<String> {
    users.into_iter()
        .filter(|user| user.is_active)
        .map(|user| user.get_display_name())
        .collect()
}

pub async fn fetch_user_async(id: UserId) -> Result<User, String> {
    // Simulate async operation
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Mock user creation
    User::new(id, format!("User {}", id), format!("user{}@example.com", id))
}

// Macro
macro_rules! create_user {
    ($id:expr, $name:expr, $email:expr) => {
        User::new($id, $name.to_string(), $email.to_string())
    };
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "John Doe".to_string(), "john@example.com".to_string());
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert!(user.is_active);
    }
    
    #[test]
    fn test_user_validation() {
        let user = User::new(1, "".to_string(), "invalid-email".to_string());
        assert!(user.is_err());
    }
    
    #[test]
    fn test_permission_management() {
        let mut user = User::create_guest();
        user.add_permission("read".to_string());
        assert!(user.permissions.contains(&"read".to_string()));
        
        let removed = user.remove_permission("read");
        assert!(removed);
        assert!(!user.permissions.contains(&"read".to_string()));
    }
}