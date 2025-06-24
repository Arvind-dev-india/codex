using System;

namespace UtilityLibrary
{
    public class MathHelper
    {
        public int Add(int x, int y)
        {
            return x + y;
        }
        
        public int Multiply(int x, int y)
        {
            return x * y;
        }
        
        public void DisplayResult(int value)
        {
            Console.WriteLine($"The answer is: {value}");
        }
    }
}