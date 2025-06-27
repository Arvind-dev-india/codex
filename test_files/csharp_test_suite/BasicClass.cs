using System;
using System.Collections.Generic;

namespace TestNamespace
{
    /// <summary>
    /// A basic class for testing C# parsing
    /// </summary>
    public class BasicClass
    {
        private int _privateField;
        public string PublicProperty { get; set; }
        
        public BasicClass()
        {
            _privateField = 0;
            PublicProperty = "default";
        }
        
        public BasicClass(int value, string text)
        {
            _privateField = value;
            PublicProperty = text;
        }
        
        public int Add(int a, int b)
        {
            return a + b;
        }
        
        public void PrintInfo()
        {
            Console.WriteLine($"Field: {_privateField}, Property: {PublicProperty}");
        }
        
        private bool IsValid()
        {
            return _privateField >= 0 && !string.IsNullOrEmpty(PublicProperty);
        }
        
        public static void StaticMethod()
        {
            Console.WriteLine("Static method called");
        }
    }
}