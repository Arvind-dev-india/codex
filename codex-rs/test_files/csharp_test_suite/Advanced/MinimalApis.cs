using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace TestApp.Advanced;

// Minimal API style patterns (C# 10+)
public class MinimalApiExamples
{
    // Target-typed new expressions
    private readonly Dictionary<string, object> _data = new();
    private readonly List<string> _logs = new();
    
    // Lambda improvements (C# 10)
    public Func<string, bool> IsValidEmail => email => !string.IsNullOrEmpty(email) && email.Contains('@');
    
    public Action<string> LogMessage => message => _logs.Add($"[{DateTime.Now}] {message}");
    
    // Constant interpolated strings (C# 10)
    private const string DefaultMessage = $"Welcome to the application";
    
    // Extended property patterns (C# 10)
    public string GetUserStatus(object user) => user switch
    {
        { } when user.ToString()?.Length > 0 => "Valid user",
        null => "Null user",
        _ => "Invalid user"
    };
    
    // Caller argument expressions (C# 10)
    public void ValidateArgument<T>(T argument, [System.Runtime.CompilerServices.CallerArgumentExpression("argument")] string? paramName = null)
    {
        if (argument == null)
        {
            throw new ArgumentNullException(paramName);
        }
    }
    
    // Generic attributes (C# 11)
    [GenericAttribute<string>]
    public class AttributedClass
    {
        [GenericAttribute<int>]
        public int Value { get; set; }
    }
    
    // Required members (C# 11)
    public class RequiredMembersExample
    {
        public required string Name { get; init; }
        public required int Id { get; init; }
        public string? Description { get; init; }
    }
    
    // Raw string literals (C# 11)
    public string GetJsonTemplate() => """
        {
            "name": "{{name}}",
            "id": {{id}},
            "active": true
        }
        """;
    
    // List patterns (C# 11)
    public string AnalyzeList<T>(T[] items) => items switch
    {
        [] => "Empty list",
        [var single] => $"Single item: {single}",
        [var first, var second] => $"Two items: {first}, {second}",
        [var first, .., var last] => $"Multiple items from {first} to {last}",
        _ => "Unknown pattern"
    };
    
    // Static abstract members in interfaces (C# 11)
    public interface IStaticMethods<T> where T : IStaticMethods<T>
    {
        static abstract T Create();
        static abstract string GetTypeName();
    }
    
    public class StaticMethodsImpl : IStaticMethods<StaticMethodsImpl>
    {
        public static StaticMethodsImpl Create() => new();
        public static string GetTypeName() => nameof(StaticMethodsImpl);
    }
    
    // Newlines in string interpolations (C# 11)
    public string FormatMultilineData(string name, int age, string city) => $"""
        Name: {name}
        Age: {age}
        City: {city}
        """;
    
    // Span pattern matching (C# 11)
    public bool StartsWithHello(ReadOnlySpan<char> text) => text switch
    {
        ['H', 'e', 'l', 'l', 'o', ..] => true,
        _ => false
    };
}

// Generic attribute definition (C# 11)
[AttributeUsage(AttributeTargets.All)]
public class GenericAttribute<T> : Attribute
{
    public T? Value { get; set; }
    
    public GenericAttribute() { }
    
    public GenericAttribute(T value)
    {
        Value = value;
    }
}

// File-scoped types (C# 11)
file class FileLocalClass
{
    public string GetMessage() => "This class is only visible within this file";
}

// UTF-8 string literals (C# 11)
public class Utf8Examples
{
    public ReadOnlySpan<byte> GetUtf8Bytes() => "Hello, World!"u8;
    
    public void ProcessUtf8Data()
    {
        ReadOnlySpan<byte> data = "Sample data"u8;
        // Process UTF-8 data directly
    }
}