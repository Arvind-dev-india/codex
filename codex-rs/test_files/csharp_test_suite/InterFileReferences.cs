using System;

namespace TestNamespace
{
    public class Calculator
    {
        private BasicClass _helper;
        private ICalculator _calculator;
        
        public Calculator()
        {
            _helper = new BasicClass();
            _calculator = new AddCalculator();
        }
        
        public int PerformCalculation(int a, int b)
        {
            // Call method from BasicClass
            int sum = _helper.Add(a, b);
            
            // Call method from interface implementation
            int result = _calculator.Calculate(sum, 10);
            
            // Call static method
            BasicClass.StaticMethod();
            
            return result;
        }
        
        public void UseMultiplyCalculator()
        {
            var multiplier = new MultiplyCalculator();
            int result = multiplier.Calculate(5, 3);
            multiplier.Reset();
        }
        
        public void CreateAndUseObjects()
        {
            var basic = new BasicClass(42, "test");
            basic.PrintInfo();
            
            var adder = new AddCalculator();
            adder.Calculate(1, 2);
            adder.Reset();
        }
    }
}