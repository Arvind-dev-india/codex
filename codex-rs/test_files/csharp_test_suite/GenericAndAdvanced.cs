using System;
using System.Collections.Generic;
using System.Linq;

namespace TestNamespace.Advanced
{
    public class GenericRepository<T> where T : class
    {
        private List<T> _items;
        
        public GenericRepository()
        {
            _items = new List<T>();
        }
        
        public void Add(T item)
        {
            _items.Add(item);
        }
        
        public T Get(int index)
        {
            return _items[index];
        }
        
        public IEnumerable<T> GetAll()
        {
            return _items.AsEnumerable();
        }
    }
    
    public class DataProcessor
    {
        private GenericRepository<BasicClass> _repository;
        
        public DataProcessor()
        {
            _repository = new GenericRepository<BasicClass>();
        }
        
        public void ProcessData()
        {
            var item = new BasicClass(100, "processed");
            _repository.Add(item);
            
            var retrieved = _repository.Get(0);
            retrieved.PrintInfo();
            
            var allItems = _repository.GetAll();
            foreach (var basicItem in allItems)
            {
                basicItem.PrintInfo();
            }
        }
        
        public void UseCalculators()
        {
            var calculators = new List<ICalculator>
            {
                new AddCalculator(),
                new MultiplyCalculator()
            };
            
            foreach (var calc in calculators)
            {
                calc.Calculate(5, 10);
                calc.Reset();
            }
        }
    }
}