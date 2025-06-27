using System;
using System.Collections.Generic;
using System.Linq;
using TestApp.Models;

namespace TestApp.Data
{
    public class InMemoryRepository<T> : IRepository<T> where T : class
    {
        protected readonly List<T> _entities;
        protected readonly Func<T, int> _getIdFunc;

        public InMemoryRepository(Func<T, int> getIdFunc)
        {
            _entities = new List<T>();
            _getIdFunc = getIdFunc ?? throw new ArgumentNullException(nameof(getIdFunc));
        }

        public virtual void Add(T entity)
        {
            if (entity == null)
                throw new ArgumentNullException(nameof(entity));

            _entities.Add(entity);
            LogOperation("Added", entity);
        }

        public virtual T GetById(int id)
        {
            return _entities.FirstOrDefault(e => _getIdFunc(e) == id);
        }

        public virtual List<T> GetAll()
        {
            return new List<T>(_entities);
        }

        public virtual void Update(T entity)
        {
            if (entity == null)
                throw new ArgumentNullException(nameof(entity));

            var id = _getIdFunc(entity);
            var existingEntity = GetById(id);
            if (existingEntity == null)
                throw new InvalidOperationException($"Entity with ID {id} not found");

            var index = _entities.IndexOf(existingEntity);
            _entities[index] = entity;
            LogOperation("Updated", entity);
        }

        public virtual void Delete(int id)
        {
            var entity = GetById(id);
            if (entity != null)
            {
                _entities.Remove(entity);
                LogOperation("Deleted", entity);
            }
        }

        public virtual bool Exists(int id)
        {
            return GetById(id) != null;
        }

        protected virtual void LogOperation(string operation, T entity)
        {
            var id = _getIdFunc(entity);
            Console.WriteLine($"{operation} {typeof(T).Name} with ID {id}");
        }
    }

    public class UserRepository : InMemoryRepository<User>, IUserRepository
    {
        public UserRepository() : base(u => u.Id)
        {
        }

        public User GetByEmail(string email)
        {
            if (string.IsNullOrWhiteSpace(email))
                return null;

            return _entities.FirstOrDefault(u => 
                u.Email.Equals(email, StringComparison.OrdinalIgnoreCase));
        }

        public List<User> GetActiveUsers()
        {
            return _entities.Where(u => u.Orders.Any(o => 
                o.Status != OrderStatus.Delivered && 
                o.Status != OrderStatus.Cancelled)).ToList();
        }

        protected override void LogOperation(string operation, User entity)
        {
            Console.WriteLine($"{operation} User: {entity.Name} ({entity.Email})");
        }
    }

    public class OrderRepository : InMemoryRepository<Order>, IOrderRepository
    {
        public OrderRepository() : base(o => o.Id)
        {
        }

        public List<Order> GetOrdersByUser(int userId)
        {
            return _entities.Where(o => o.UserId == userId).ToList();
        }

        public List<Order> GetOrdersByStatus(OrderStatus status)
        {
            return _entities.Where(o => o.Status == status).ToList();
        }

        public List<Order> GetOrdersByDateRange(DateTime startDate, DateTime endDate)
        {
            return _entities.Where(o => 
                o.OrderDate >= startDate && 
                o.OrderDate <= endDate).ToList();
        }

        protected override void LogOperation(string operation, Order entity)
        {
            Console.WriteLine($"{operation} Order: {entity.Id} for User {entity.UserId}");
        }
    }
}