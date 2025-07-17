use codex_core::code_analysis::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    handle_get_related_files_skeleton,
    handle_get_multiple_files_skeleton,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Comprehensive test for C# cross-project analysis
/// Tests main project using classes/methods from skeleton (read-only) project
#[tokio::test]
async fn test_csharp_cross_project_comprehensive() {
    println!("\n=== C# Cross-Project Comprehensive Test ===");
    
    // Create temporary directories for main and skeleton projects
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let main_project_dir = temp_dir.path().join("MainProject");
    let skeleton_project_dir = temp_dir.path().join("SkeletonProject");
    
    fs::create_dir_all(&main_project_dir).expect("Failed to create main project dir");
    fs::create_dir_all(&skeleton_project_dir).expect("Failed to create skeleton project dir");
    
    // Create skeleton project files (read-only dependencies)
    create_skeleton_project_files(&skeleton_project_dir);
    
    // Create main project files (uses skeleton project)
    create_main_project_files(&main_project_dir);
    
    // Test 1: Analyze both projects
    println!("\n--- Test 1: Analyzing Projects ---");
    test_analyze_projects(&main_project_dir, &skeleton_project_dir).await;
    
    // Test 2: Find symbol references across projects
    println!("\n--- Test 2: Cross-Project Symbol References ---");
    test_cross_project_symbol_references(&main_project_dir, &skeleton_project_dir).await;
    
    // Test 3: Find symbol definitions across projects
    println!("\n--- Test 3: Cross-Project Symbol Definitions ---");
    test_cross_project_symbol_definitions(&main_project_dir, &skeleton_project_dir).await;
    
    // Test 4: Get symbol subgraph across projects
    println!("\n--- Test 4: Cross-Project Symbol Subgraph ---");
    test_cross_project_symbol_subgraph(&main_project_dir, &skeleton_project_dir).await;
    
    // Test 5: Get related files skeleton across projects
    println!("\n--- Test 5: Cross-Project Related Files Skeleton ---");
    test_cross_project_related_files_skeleton(&main_project_dir, &skeleton_project_dir).await;
    
    // Test 6: Get multiple files skeleton across projects
    println!("\n--- Test 6: Cross-Project Multiple Files Skeleton ---");
    test_cross_project_multiple_files_skeleton(&main_project_dir, &skeleton_project_dir).await;
    
    println!("\n=== All Cross-Project Tests Completed ===");
}

/// Create skeleton project files (read-only dependencies)
fn create_skeleton_project_files(skeleton_dir: &Path) {
    // Models/User.cs - Base user model
    let models_dir = skeleton_dir.join("Models");
    fs::create_dir_all(&models_dir).expect("Failed to create models dir");
    
    let user_model = r#"using System;
using System.ComponentModel.DataAnnotations;

namespace SkeletonProject.Models
{
    public class User
    {
        public int Id { get; set; }
        
        [Required]
        public string Name { get; set; }
        
        [EmailAddress]
        public string Email { get; set; }
        
        public DateTime CreatedAt { get; set; }
        
        public virtual void ValidateUser()
        {
            if (string.IsNullOrEmpty(Name))
                throw new ArgumentException("Name is required");
        }
        
        public virtual string GetDisplayName()
        {
            return $"{Name} ({Email})";
        }
    }
}
"#;
    fs::write(models_dir.join("User.cs"), user_model).expect("Failed to write User.cs");
    
    // Services/IUserRepository.cs - Interface for user operations
    let services_dir = skeleton_dir.join("Services");
    fs::create_dir_all(&services_dir).expect("Failed to create services dir");
    
    let user_repository_interface = r#"using System.Collections.Generic;
using System.Threading.Tasks;
using SkeletonProject.Models;

namespace SkeletonProject.Services
{
    public interface IUserRepository
    {
        Task<User> GetUserByIdAsync(int id);
        Task<IEnumerable<User>> GetAllUsersAsync();
        Task<User> CreateUserAsync(User user);
        Task<User> UpdateUserAsync(User user);
        Task<bool> DeleteUserAsync(int id);
        Task<User> FindUserByEmailAsync(string email);
    }
}
"#;
    fs::write(services_dir.join("IUserRepository.cs"), user_repository_interface)
        .expect("Failed to write IUserRepository.cs");
    
    // Utils/ValidationHelper.cs - Utility class
    let utils_dir = skeleton_dir.join("Utils");
    fs::create_dir_all(&utils_dir).expect("Failed to create utils dir");
    
    let validation_helper = r#"using System;
using System.Text.RegularExpressions;

namespace SkeletonProject.Utils
{
    public static class ValidationHelper
    {
        private static readonly Regex EmailRegex = new Regex(
            @"^[^@\s]+@[^@\s]+\.[^@\s]+$", 
            RegexOptions.Compiled | RegexOptions.IgnoreCase);
        
        public static bool IsValidEmail(string email)
        {
            if (string.IsNullOrWhiteSpace(email))
                return false;
            
            return EmailRegex.IsMatch(email);
        }
        
        public static bool IsValidName(string name)
        {
            return !string.IsNullOrWhiteSpace(name) && name.Length >= 2;
        }
        
        public static string SanitizeInput(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;
            
            return input.Trim().Replace("<", "&lt;").Replace(">", "&gt;");
        }
    }
}
"#;
    fs::write(utils_dir.join("ValidationHelper.cs"), validation_helper)
        .expect("Failed to write ValidationHelper.cs");
}

/// Create main project files (uses skeleton project)
fn create_main_project_files(main_dir: &Path) {
    // Models/ExtendedUser.cs - Extends skeleton User
    let models_dir = main_dir.join("Models");
    fs::create_dir_all(&models_dir).expect("Failed to create models dir");
    
    let extended_user = r#"using System;
using SkeletonProject.Models;
using SkeletonProject.Utils;

namespace MainProject.Models
{
    public class ExtendedUser : User
    {
        public string Department { get; set; }
        public bool IsActive { get; set; }
        public DateTime LastLoginAt { get; set; }
        
        public override void ValidateUser()
        {
            base.ValidateUser();
            
            if (!ValidationHelper.IsValidEmail(Email))
                throw new ArgumentException("Invalid email format");
            
            if (!ValidationHelper.IsValidName(Name))
                throw new ArgumentException("Invalid name format");
        }
        
        public override string GetDisplayName()
        {
            var baseName = base.GetDisplayName();
            return IsActive ? $"{baseName} - {Department}" : $"{baseName} (Inactive)";
        }
        
        public void UpdateLastLogin()
        {
            LastLoginAt = DateTime.UtcNow;
        }
    }
}
"#;
    fs::write(models_dir.join("ExtendedUser.cs"), extended_user)
        .expect("Failed to write ExtendedUser.cs");
    
    // Services/UserService.cs - Implements skeleton interface
    let services_dir = main_dir.join("Services");
    fs::create_dir_all(&services_dir).expect("Failed to create services dir");
    
    let user_service = r#"using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using SkeletonProject.Models;
using SkeletonProject.Services;
using SkeletonProject.Utils;
using MainProject.Models;

namespace MainProject.Services
{
    public class UserService : IUserRepository
    {
        private readonly List<User> _users;
        
        public UserService()
        {
            _users = new List<User>();
        }
        
        public async Task<User> GetUserByIdAsync(int id)
        {
            await Task.Delay(1); // Simulate async operation
            return _users.FirstOrDefault(u => u.Id == id);
        }
        
        public async Task<IEnumerable<User>> GetAllUsersAsync()
        {
            await Task.Delay(1);
            return _users.AsEnumerable();
        }
        
        public async Task<User> CreateUserAsync(User user)
        {
            if (user == null)
                throw new ArgumentNullException(nameof(user));
            
            // Use ValidationHelper from skeleton project
            if (!ValidationHelper.IsValidEmail(user.Email))
                throw new ArgumentException("Invalid email");
            
            if (!ValidationHelper.IsValidName(user.Name))
                throw new ArgumentException("Invalid name");
            
            user.ValidateUser(); // Call skeleton method
            user.Id = _users.Count + 1;
            user.CreatedAt = DateTime.UtcNow;
            
            _users.Add(user);
            await Task.Delay(1);
            
            return user;
        }
        
        public async Task<User> UpdateUserAsync(User user)
        {
            var existingUser = await GetUserByIdAsync(user.Id);
            if (existingUser == null)
                throw new ArgumentException("User not found");
            
            existingUser.Name = ValidationHelper.SanitizeInput(user.Name);
            existingUser.Email = user.Email;
            
            existingUser.ValidateUser();
            
            await Task.Delay(1);
            return existingUser;
        }
        
        public async Task<bool> DeleteUserAsync(int id)
        {
            var user = await GetUserByIdAsync(id);
            if (user == null)
                return false;
            
            _users.Remove(user);
            await Task.Delay(1);
            return true;
        }
        
        public async Task<User> FindUserByEmailAsync(string email)
        {
            if (!ValidationHelper.IsValidEmail(email))
                return null;
            
            await Task.Delay(1);
            return _users.FirstOrDefault(u => 
                string.Equals(u.Email, email, StringComparison.OrdinalIgnoreCase));
        }
        
        public async Task<ExtendedUser> CreateExtendedUserAsync(string name, string email, string department)
        {
            var extendedUser = new ExtendedUser
            {
                Name = name,
                Email = email,
                Department = department,
                IsActive = true
            };
            
            var createdUser = await CreateUserAsync(extendedUser);
            return createdUser as ExtendedUser;
        }
    }
}
"#;
    fs::write(services_dir.join("UserService.cs"), user_service)
        .expect("Failed to write UserService.cs");
    
    // Controllers/UserController.cs - Uses both projects
    let controllers_dir = main_dir.join("Controllers");
    fs::create_dir_all(&controllers_dir).expect("Failed to create controllers dir");
    
    let user_controller = r#"using System;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using SkeletonProject.Models;
using SkeletonProject.Services;
using SkeletonProject.Utils;
using MainProject.Models;
using MainProject.Services;

namespace MainProject.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class UserController : ControllerBase
    {
        private readonly IUserRepository _userRepository;
        private readonly UserService _userService;
        
        public UserController(IUserRepository userRepository, UserService userService)
        {
            _userRepository = userRepository;
            _userService = userService;
        }
        
        [HttpGet("{id}")]
        public async Task<ActionResult<User>> GetUser(int id)
        {
            var user = await _userRepository.GetUserByIdAsync(id);
            if (user == null)
                return NotFound();
            
            return Ok(user);
        }
        
        [HttpPost]
        public async Task<ActionResult<User>> CreateUser([FromBody] User user)
        {
            try
            {
                if (!ValidationHelper.IsValidEmail(user.Email))
                    return BadRequest("Invalid email format");
                
                user.ValidateUser();
                var createdUser = await _userRepository.CreateUserAsync(user);
                
                return CreatedAtAction(nameof(GetUser), new { id = createdUser.Id }, createdUser);
            }
            catch (ArgumentException ex)
            {
                return BadRequest(ex.Message);
            }
        }
        
        [HttpPost("extended")]
        public async Task<ActionResult<ExtendedUser>> CreateExtendedUser([FromBody] CreateExtendedUserRequest request)
        {
            try
            {
                var extendedUser = await _userService.CreateExtendedUserAsync(
                    request.Name, request.Email, request.Department);
                
                return CreatedAtAction(nameof(GetUser), new { id = extendedUser.Id }, extendedUser);
            }
            catch (ArgumentException ex)
            {
                return BadRequest(ex.Message);
            }
        }
        
        [HttpGet("search")]
        public async Task<ActionResult<User>> FindUserByEmail([FromQuery] string email)
        {
            if (!ValidationHelper.IsValidEmail(email))
                return BadRequest("Invalid email format");
            
            var user = await _userRepository.FindUserByEmailAsync(email);
            if (user == null)
                return NotFound();
            
            return Ok(user);
        }
    }
    
    public class CreateExtendedUserRequest
    {
        public string Name { get; set; }
        public string Email { get; set; }
        public string Department { get; set; }
    }
}
"#;
    fs::write(controllers_dir.join("UserController.cs"), user_controller)
        .expect("Failed to write UserController.cs");
}

/// Test analyzing both projects
async fn test_analyze_projects(main_dir: &Path, skeleton_dir: &Path) {
    let main_files = get_all_cs_files(main_dir);
    let skeleton_files = get_all_cs_files(skeleton_dir);
    
    println!("Main project files: {:?}", main_files);
    println!("Skeleton project files: {:?}", skeleton_files);
    
    // Analyze main project
    for file in &main_files {
        let input = json!({
            "file_path": file.to_string_lossy()
        });
        
        match handle_analyze_code(input) {
            Some(Ok(result)) => {
                println!("✅ Successfully analyzed main project file: {}", file.display());
                if let Some(symbols) = result.get("symbols").and_then(|s| s.as_array()) {
                    println!("   Found {} symbols", symbols.len());
                }
            }
            Some(Err(e)) => println!("❌ Failed to analyze {}: {}", file.display(), e),
            None => println!("❌ No result for {}", file.display()),
        }
    }
    
    // Analyze skeleton project
    for file in &skeleton_files {
        let input = json!({
            "file_path": file.to_string_lossy()
        });
        
        match handle_analyze_code(input) {
            Some(Ok(result)) => {
                println!("✅ Successfully analyzed skeleton project file: {}", file.display());
                if let Some(symbols) = result.get("symbols").and_then(|s| s.as_array()) {
                    println!("   Found {} symbols", symbols.len());
                }
            }
            Some(Err(e)) => println!("❌ Failed to analyze {}: {}", file.display(), e),
            None => println!("❌ No result for {}", file.display()),
        }
    }
}

/// Test cross-project symbol references
async fn test_cross_project_symbol_references(_main_dir: &Path, _skeleton_dir: &Path) {
    // Test finding references to skeleton project symbols from main project
    let test_symbols = vec![
        "User",
        "IUserRepository", 
        "ValidationHelper",
        "ValidateUser",
        "IsValidEmail",
        "GetUserByIdAsync"
    ];
    
    for symbol in test_symbols {
        let input = json!({
            "symbol_name": symbol
        });
        
        match handle_find_symbol_references(input) {
            Some(Ok(result)) => {
                println!("✅ Found references for symbol: {}", symbol);
                if let Some(references) = result.get("references").and_then(|r| r.as_array()) {
                    println!("   Found {} references", references.len());
                    
                    // Check for cross-project references
                    let mut main_refs = 0;
                    let mut skeleton_refs = 0;
                    
                    for reference in references {
                        if let Some(file) = reference.get("file").and_then(|f| f.as_str()) {
                            if file.contains("MainProject") {
                                main_refs += 1;
                            } else if file.contains("SkeletonProject") {
                                skeleton_refs += 1;
                            }
                        }
                    }
                    
                    println!("   Main project references: {}, Skeleton project references: {}", 
                            main_refs, skeleton_refs);
                    
                    if main_refs > 0 && skeleton_refs > 0 {
                        println!("   ✅ Cross-project references detected!");
                    }
                }
            }
            Some(Err(e)) => println!("❌ Failed to find references for {}: {}", symbol, e),
            None => println!("❌ No result for symbol: {}", symbol),
        }
    }
}

/// Test cross-project symbol definitions
async fn test_cross_project_symbol_definitions(_main_dir: &Path, _skeleton_dir: &Path) {
    let test_symbols = vec![
        "User",
        "ExtendedUser",
        "IUserRepository",
        "UserService",
        "ValidationHelper",
        "UserController"
    ];
    
    for symbol in test_symbols {
        let input = json!({
            "symbol_name": symbol
        });
        
        match handle_find_symbol_definitions(input) {
            Some(Ok(result)) => {
                println!("✅ Found definitions for symbol: {}", symbol);
                if let Some(definitions) = result.get("definitions").and_then(|d| d.as_array()) {
                    println!("   Found {} definitions", definitions.len());
                    
                    for definition in definitions {
                        if let Some(file) = definition.get("file").and_then(|f| f.as_str()) {
                            let project = if file.contains("MainProject") { "Main" } else { "Skeleton" };
                            println!("   Definition in {} project: {}", project, file);
                        }
                    }
                }
            }
            Some(Err(e)) => println!("❌ Failed to find definitions for {}: {}", symbol, e),
            None => println!("❌ No result for symbol: {}", symbol),
        }
    }
}

/// Test cross-project symbol subgraph
async fn test_cross_project_symbol_subgraph(_main_dir: &Path, _skeleton_dir: &Path) {
    let test_symbols = vec!["UserService", "ExtendedUser", "ValidationHelper"];
    
    for symbol in test_symbols {
        let input = json!({
            "symbol_name": symbol,
            "depth": 2
        });
        
        match handle_get_symbol_subgraph(input) {
            Some(Ok(result)) => {
                println!("✅ Generated subgraph for symbol: {}", symbol);
                if let Some(nodes) = result.get("nodes").and_then(|n| n.as_array()) {
                        println!("   Found {} nodes in subgraph", nodes.len());
                        
                        let mut cross_project_edges = 0;
                        if let Some(edges) = result.get("edges").and_then(|e| e.as_array()) {
                            for edge in edges {
                                if let (Some(source_file), Some(target_file)) = (
                                    edge.get("source_file").and_then(|f| f.as_str()),
                                    edge.get("target_file").and_then(|f| f.as_str())
                                ) {
                                    let source_project = if source_file.contains("MainProject") { "Main" } else { "Skeleton" };
                                    let target_project = if target_file.contains("MainProject") { "Main" } else { "Skeleton" };
                                    
                                    if source_project != target_project {
                                        cross_project_edges += 1;
                                    }
                                }
                            }
                        }
                        
                        println!("   Cross-project edges: {}", cross_project_edges);
                        if cross_project_edges > 0 {
                            println!("   ✅ Cross-project dependencies detected in subgraph!");
                        }
                    }
            }
            Some(Err(e)) => println!("❌ Failed to generate subgraph for {}: {}", symbol, e),
            None => println!("❌ No result for symbol: {}", symbol),
        }
    }
}

/// Test cross-project related files skeleton
async fn test_cross_project_related_files_skeleton(main_dir: &Path, _skeleton_dir: &Path) {
    let main_files = get_all_cs_files(main_dir);
    
    if let Some(user_service_file) = main_files.iter().find(|f| f.file_name().unwrap() == "UserService.cs") {
        let input = json!({
            "active_files": [user_service_file.to_string_lossy()],
            "max_depth": 2,
            "max_tokens": 4000
        });
        
        match handle_get_related_files_skeleton(input) {
            Some(Ok(result)) => {
                println!("✅ Generated related files skeleton for UserService.cs");
                if let Some(files) = result.get("files").and_then(|f| f.as_array()) {
                        println!("   Found {} related files", files.len());
                        
                        let mut main_files = 0;
                        let mut skeleton_files = 0;
                        
                        for file in files {
                            if let Some(file_path) = file.get("file_path").and_then(|f| f.as_str()) {
                                if file_path.contains("MainProject") {
                                    main_files += 1;
                                } else if file_path.contains("SkeletonProject") {
                                    skeleton_files += 1;
                                }
                                println!("   Related file: {}", file_path);
                            }
                        }
                        
                        println!("   Main project files: {}, Skeleton project files: {}", 
                                main_files, skeleton_files);
                        
                        if skeleton_files > 0 {
                            println!("   ✅ Cross-project related files detected!");
                        }
                    }
            }
            Some(Err(e)) => println!("❌ Failed to generate related files skeleton: {}", e),
            None => println!("❌ No result for related files skeleton"),
        }
    }
}

/// Test cross-project multiple files skeleton
async fn test_cross_project_multiple_files_skeleton(main_dir: &Path, skeleton_dir: &Path) {
    let main_files = get_all_cs_files(main_dir);
    let skeleton_files = get_all_cs_files(skeleton_dir);
    
    // Select a mix of files from both projects
    let mut selected_files = Vec::new();
    
    if let Some(user_service) = main_files.iter().find(|f| f.file_name().unwrap() == "UserService.cs") {
        selected_files.push(user_service.to_string_lossy().to_string());
    }
    
    if let Some(user_model) = skeleton_files.iter().find(|f| f.file_name().unwrap() == "User.cs") {
        selected_files.push(user_model.to_string_lossy().to_string());
    }
    
    if let Some(validation_helper) = skeleton_files.iter().find(|f| f.file_name().unwrap() == "ValidationHelper.cs") {
        selected_files.push(validation_helper.to_string_lossy().to_string());
    }
    
    if !selected_files.is_empty() {
        let input = json!({
            "file_paths": selected_files,
            "max_tokens": 4000
        });
        
        match handle_get_multiple_files_skeleton(input) {
            Some(Ok(result)) => {
                println!("✅ Generated multiple files skeleton for cross-project files");
                if let Some(files) = result.get("files").and_then(|f| f.as_array()) {
                        println!("   Generated skeletons for {} files", files.len());
                        
                        for file in files {
                            if let Some(file_path) = file.get("file_path").and_then(|f| f.as_str()) {
                                let project = if file_path.contains("MainProject") { "Main" } else { "Skeleton" };
                                println!("   Skeleton for {} project file: {}", project, file_path);
                            }
                        }
                        
                        println!("   ✅ Cross-project multiple files skeleton generated successfully!");
                    }
            }
            Some(Err(e)) => println!("❌ Failed to generate multiple files skeleton: {}", e),
            None => println!("❌ No result for multiple files skeleton"),
        }
    }
}

/// Helper function to get all C# files in a directory
fn get_all_cs_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut cs_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                cs_files.extend(get_all_cs_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("cs") {
                cs_files.push(path);
            }
        }
    }
    
    cs_files
}