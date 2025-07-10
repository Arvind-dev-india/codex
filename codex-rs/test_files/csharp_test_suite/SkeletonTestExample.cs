using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace SkeletonTest.Examples
{
    /// <summary>
    /// A comprehensive test class for skeleton generation verification
    /// </summary>
    public class SkeletonTestExample : IDisposable
    {
        private readonly string _privateField;
        private static readonly Dictionary<string, int> _staticData = new();
        
        public string PublicProperty { get; set; }
        public int ReadOnlyProperty { get; }
        protected virtual bool ProtectedProperty { get; set; }
        
        // Event declaration
        public event EventHandler<string> DataChanged;
        
        // Default constructor
        public SkeletonTestExample()
        {
            _privateField = "default";
            PublicProperty = "initialized";
            ReadOnlyProperty = 42;
        }
        
        // Parameterized constructor
        public SkeletonTestExample(string initialValue, int readOnlyValue) : this()
        {
            _privateField = initialValue ?? throw new ArgumentNullException(nameof(initialValue));
            ReadOnlyProperty = readOnlyValue;
        }
        
        // Public method with multiple parameters
        public async Task<List<T>> ProcessDataAsync<T>(IEnumerable<T> input, Func<T, bool> predicate) where T : class
        {
            if (input == null) throw new ArgumentNullException(nameof(input));
            if (predicate == null) throw new ArgumentNullException(nameof(predicate));
            
            var result = new List<T>();
            
            await Task.Run(() =>
            {
                foreach (var item in input.Where(predicate))
                {
                    result.Add(item);
                    OnDataChanged($"Processed: {item}");
                }
            });
            
            return result;
        }
        
        // Static method
        public static void InitializeStaticData(Dictionary<string, int> data)
        {
            if (data == null) return;
            
            _staticData.Clear();
            foreach (var kvp in data)
            {
                _staticData[kvp.Key] = kvp.Value;
            }
        }
        
        // Protected virtual method
        protected virtual void OnDataChanged(string message)
        {
            DataChanged?.Invoke(this, message);
        }
        
        // Private method with complex logic
        private bool ValidateInput(object input)
        {
            if (input == null) return false;
            
            return input switch
            {
                string s => !string.IsNullOrWhiteSpace(s),
                int i => i > 0,
                IEnumerable<object> collection => collection.Any(),
                _ => true
            };
        }
        
        // Operator overload
        public static SkeletonTestExample operator +(SkeletonTestExample left, SkeletonTestExample right)
        {
            if (left == null || right == null) return null;
            
            return new SkeletonTestExample(
                left._privateField + right._privateField,
                left.ReadOnlyProperty + right.ReadOnlyProperty
            );
        }
        
        // Indexer
        public string this[int index]
        {
            get
            {
                if (index < 0 || index >= _privateField.Length)
                    throw new IndexOutOfRangeException();
                return _privateField[index].ToString();
            }
        }
        
        // Finalizer
        ~SkeletonTestExample()
        {
            Dispose(false);
        }
        
        // IDisposable implementation
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
        
        protected virtual void Dispose(bool disposing)
        {
            if (disposing)
            {
                // Dispose managed resources
                DataChanged = null;
            }
            // Dispose unmanaged resources
        }
    }
    
    // Interface for testing
    public interface ISkeletonTestService
    {
        Task<bool> ProcessAsync(string data);
        event Action<string> StatusChanged;
        string Status { get; }
    }
    
    // Abstract class for testing
    public abstract class BaseSkeletonTest
    {
        protected abstract void Initialize();
        public virtual void Start() => Initialize();
    }
    
    // Enum for testing
    public enum SkeletonTestStatus
    {
        None = 0,
        Initializing = 1,
        Processing = 2,
        Completed = 3,
        Failed = -1
    }
    
    // Record for testing (C# 9+ feature)
    public record SkeletonTestRecord(string Name, int Value)
    {
        public string DisplayName => $"{Name}: {Value}";
    }
    
    // Struct for testing
    public struct SkeletonTestStruct
    {
        public int X { get; set; }
        public int Y { get; set; }
        
        public SkeletonTestStruct(int x, int y)
        {
            X = x;
            Y = y;
        }
        
        public readonly double Distance => Math.Sqrt(X * X + Y * Y);
    }
}