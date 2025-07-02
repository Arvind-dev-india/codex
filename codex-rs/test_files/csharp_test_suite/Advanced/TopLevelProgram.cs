// Top-level program (C# 9) - simplified Main method
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using TestApp.Models;
using TestApp.Advanced;

// Global using statements would normally be in a separate file
// global using System.Text.Json;

Console.WriteLine("Starting top-level program...");

// Direct code execution without Main method
var users = await CreateSampleUsersAsync();
ProcessUsers(users);

// Target-typed new expressions (C# 9)
List<UserRecord> records = new()
{
    new(1, "Alice", "alice@example.com"),
    new(2, "Bob", "bob@example.com")
};

// Pattern matching with relational patterns (C# 9)
foreach (var user in users)
{
    var category = user.Id switch
    {
        < 10 => "Single digit",
        >= 10 and < 100 => "Double digit", 
        >= 100 => "Triple digit or more",
        _ => "Unknown"
    };
    
    Console.WriteLine($"User {user.Name} has {category} ID");
}

// Local functions in top-level program
static async Task<List<User>> CreateSampleUsersAsync()
{
    await Task.Delay(10);
    
    return new List<User>
    {
        new(1, "John Doe", "john@example.com"),
        new(2, "Jane Smith", "jane@example.com"),
        new(15, "Mike Johnson", "mike@example.com"),
        new(150, "Sarah Wilson", "sarah@example.com")
    };
}

static void ProcessUsers(IEnumerable<User> users)
{
    // Range and index operators (C# 8)
    var userArray = users.ToArray();
    
    if (userArray.Length > 0)
    {
        var firstUser = userArray[0];           // First element
        var lastUser = userArray[^1];           // Last element
        var middleUsers = userArray[1..^1];     // All except first and last
        
        Console.WriteLine($"First: {firstUser.Name}");
        Console.WriteLine($"Last: {lastUser.Name}");
        Console.WriteLine($"Middle users count: {middleUsers.Length}");
    }
    
    // Using declarations (C# 8)
    using var fileStream = new FileStream("sample.txt");
    
    // Switch expressions with property patterns
    foreach (var user in users)
    {
        var status = user switch
        {
            { Email: var email } when email.Contains("@example.com") => "Example user",
            { Name.Length: > 10 } => "Long name user",
            { Id: > 100 } => "High ID user",
            _ => "Regular user"
        };
        
        Console.WriteLine($"{user.Name}: {status}");
    }
}

// Static local functions
static string FormatUserInfo(User user) => $"{user.Name} ({user.Email})";

// Demonstration of init-only properties and records
var userRecord = new UserRecord(999, "Test User", "test@example.com")
{
    CreatedAt = DateTime.Now,
    Department = "Engineering"
};

Console.WriteLine($"Created user record: {userRecord}");

// With expressions for records (C# 9)
var updatedRecord = userRecord with { Department = "Marketing" };
Console.WriteLine($"Updated record: {updatedRecord}");

Console.WriteLine("Top-level program completed.");