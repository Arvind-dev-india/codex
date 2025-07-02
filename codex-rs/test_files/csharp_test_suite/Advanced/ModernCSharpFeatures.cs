using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.ComponentModel.DataAnnotations;

namespace TestApp.Advanced;

// File-scoped namespace (C# 10)
// Records (C# 9)
public record UserRecord(int Id, string Name, string Email)
{
    // Init-only properties (C# 9)
    public DateTime CreatedAt { get; init; } = DateTime.Now;
    public string? Department { get; init; } // Nullable reference types (C# 8)
}

public record struct Point(double X, double Y);

// Pattern matching and switch expressions (C# 8/9)
public class PatternMatchingExamples
{
    public string AnalyzeObject(object obj) => obj switch
    {
        string s when s.Length > 10 => "Long string",
        string s => $"Short string: {s}",
        int i when i > 0 => "Positive number",
        int i => "Non-positive number",
        UserRecord { Name: var name, Email: var email } => $"User: {name} ({email})",
        null => "Null object",
        _ => "Unknown type"
    };

    public bool IsValidUser(UserRecord? user) => user is { Name.Length: > 0, Email: not null };
    
    // Traditional method syntax for better detection
    public string AnalyzeObjectTraditional(object obj)
    {
        return obj switch
        {
            string s when s.Length > 10 => "Long string",
            string s => $"Short string: {s}",
            int i when i > 0 => "Positive number",
            int i => "Non-positive number",
            UserRecord { Name: var name, Email: var email } => $"User: {name} ({email})",
            null => "Null object",
            _ => "Unknown type"
        };
    }
}

// Async/await patterns
public class AsyncOperations
{
    public async Task<List<UserRecord>> GetUsersAsync()
    {
        await Task.Delay(100); // Simulate async operation
        
        return new List<UserRecord>
        {
            new(1, "John Doe", "john@example.com"),
            new(2, "Jane Smith", "jane@example.com")
        };
    }

    public async Task<UserRecord?> FindUserAsync(int id)
    {
        var users = await GetUsersAsync();
        return users.FirstOrDefault(u => u.Id == id);
    }

    public async IAsyncEnumerable<UserRecord> GetUsersStreamAsync()
    {
        var users = await GetUsersAsync();
        foreach (var user in users)
        {
            await Task.Delay(50);
            yield return user;
        }
    }
}

// LINQ expressions and lambda expressions
public class LinqExamples
{
    private readonly List<UserRecord> _users = new();

    public IEnumerable<UserRecord> GetActiveUsers() =>
        _users.Where(u => u.Email != null)
              .OrderBy(u => u.Name)
              .Select(u => u with { Department = "Active" });

    // Traditional method syntax for better detection
    public IEnumerable<UserRecord> GetActiveUsersTraditional()
    {
        return _users.Where(u => u.Email != null)
                    .OrderBy(u => u.Name)
                    .Select(u => u with { Department = "Active" });
    }

    public Dictionary<string, List<UserRecord>> GroupUsersByDepartment() =>
        _users.Where(u => u.Department != null)
              .GroupBy(u => u.Department!)
              .ToDictionary(g => g.Key, g => g.ToList());

    public bool HasUsersInDepartment(string department) =>
        _users.Any(u => u.Department?.Equals(department, StringComparison.OrdinalIgnoreCase) == true);

    // Complex LINQ with multiple operations
    public async Task<IEnumerable<string>> GetUserEmailsAsync(Func<UserRecord, bool> predicate)
    {
        var asyncOps = new AsyncOperations();
        var users = await asyncOps.GetUsersAsync();
        
        return users.Where(predicate)
                   .Select(u => u.Email)
                   .Where(email => !string.IsNullOrEmpty(email))
                   .Distinct()
                   .OrderBy(email => email);
    }
}

// Attributes and decorators
[Serializable]
[Obsolete("Use UserRecord instead")]
public class LegacyUser
{
    [Required]
    [StringLength(100)]
    public string Name { get; set; } = string.Empty;

    [EmailAddress]
    public string Email { get; set; } = string.Empty;

    [Range(0, 150)]
    public int Age { get; set; }
}

// Custom attributes
[AttributeUsage(AttributeTargets.Class | AttributeTargets.Method)]
public class AuditableAttribute : Attribute
{
    public string AuditLevel { get; set; } = "Info";
    public bool LogParameters { get; set; } = false;
}

[Auditable(AuditLevel = "Debug", LogParameters = true)]
public class AuditedService
{
    [Auditable(AuditLevel = "Warning")]
    public void ProcessData(string data)
    {
        Console.WriteLine($"Processing: {data}");
    }
}

// Extension methods
public static class StringExtensions
{
    public static bool IsValidEmail(this string email) =>
        !string.IsNullOrEmpty(email) && email.Contains('@') && email.Contains('.');

    public static string ToTitleCase(this string input) =>
        string.IsNullOrEmpty(input) ? input : 
        char.ToUpper(input[0]) + input[1..].ToLower();
        
    // Traditional method syntax for better detection
    public static bool IsValidEmailTraditional(this string email)
    {
        return !string.IsNullOrEmpty(email) && email.Contains('@') && email.Contains('.');
    }
}

// Generic constraints and covariance/contravariance
public interface IRepository<out T> where T : class
{
    Task<T?> GetByIdAsync(int id);
    Task<IEnumerable<T>> GetAllAsync();
}

public interface ICommandHandler<in TCommand> where TCommand : class
{
    Task HandleAsync(TCommand command);
}

public class GenericService<T, TKey> 
    where T : class, IEntity<TKey>
    where TKey : IEquatable<TKey>
{
    public async Task<T?> FindAsync(TKey id)
    {
        await Task.Delay(10);
        return default(T);
    }

    public void ProcessEntities<TDerived>(IEnumerable<TDerived> entities) 
        where TDerived : T
    {
        foreach (var entity in entities)
        {
            Console.WriteLine($"Processing entity: {entity}");
        }
    }
}

public interface IEntity<T> where T : IEquatable<T>
{
    T Id { get; }
}

// Local functions and nested methods
public class LocalFunctionExamples
{
    public int CalculateFactorial(int n)
    {
        if (n < 0) throw new ArgumentException("Number must be non-negative");
        
        return CalculateFactorialRecursive(n);
        
        static int CalculateFactorialRecursive(int num)
        {
            return num <= 1 ? 1 : num * CalculateFactorialRecursive(num - 1);
        }
    }

    public async Task<string> ProcessDataAsync(string input)
    {
        if (string.IsNullOrEmpty(input)) return string.Empty;

        var result = await ProcessInternalAsync();
        return FormatResult(result);

        async Task<string> ProcessInternalAsync()
        {
            await Task.Delay(100);
            return input.ToUpper();
        }

        static string FormatResult(string data) => $"Processed: {data}";
    }
}

// Nullable reference types and null-conditional operators
public class NullableExamples
{
    public string? ProcessUser(UserRecord? user)
    {
        return user?.Name?.ToUpper() ?? "Unknown User";
    }

    public int GetUserNameLength(UserRecord? user) => user?.Name?.Length ?? 0;

    public void UpdateUserEmail(UserRecord? user, string? newEmail)
    {
        if (user is not null && newEmail is not null)
        {
            // user = user with { Email = newEmail }; // Would need to be handled differently
            Console.WriteLine($"Updated email for {user.Name}");
        }
    }
}