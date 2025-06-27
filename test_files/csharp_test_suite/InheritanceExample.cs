using System;

namespace TestNamespace
{
    public interface ICalculator
    {
        int Calculate(int x, int y);
        void Reset();
    }
    
    public abstract class BaseCalculator : ICalculator
    {
        protected int _result;
        
        public abstract int Calculate(int x, int y);
        
        public virtual void Reset()
        {
            _result = 0;
        }
        
        protected void LogOperation(string operation)
        {
            Console.WriteLine($"Operation: {operation}");
        }
    }
    
    public class AddCalculator : BaseCalculator
    {
        public override int Calculate(int x, int y)
        {
            LogOperation("Addition");
            _result = x + y;
            return _result;
        }
    }
    
    public class MultiplyCalculator : BaseCalculator
    {
        public override int Calculate(int x, int y)
        {
            LogOperation("Multiplication");
            _result = x * y;
            return _result;
        }
        
        public override void Reset()
        {
            base.Reset();
            Console.WriteLine("Multiply calculator reset");
        }
    }
}