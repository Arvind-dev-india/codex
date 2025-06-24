using System;

namespace MathLibrary
{
    public class Calculator
    {
        public int Add(int a, int b)
        {
            return a + b;
        }
        
        public int Subtract(int a, int b)
        {
            return a - b;
        }
        
        public void PrintResult(int result)
        {
            Console.WriteLine($"Result: {result}");
        }
    }
}