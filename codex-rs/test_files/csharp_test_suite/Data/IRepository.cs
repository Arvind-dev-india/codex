using System.Collections.Generic;

namespace TestApp.Data
{
    public interface IRepository<T> where T : class
    {
        void Add(T entity);
        T GetById(int id);
        List<T> GetAll();
        void Update(T entity);
        void Delete(int id);
        bool Exists(int id);
    }

    public interface IUserRepository : IRepository<TestApp.Models.User>
    {
        TestApp.Models.User GetByEmail(string email);
        List<TestApp.Models.User> GetActiveUsers();
    }

    public interface IOrderRepository : IRepository<TestApp.Models.Order>
    {
        List<TestApp.Models.Order> GetOrdersByUser(int userId);
        List<TestApp.Models.Order> GetOrdersByStatus(TestApp.Models.OrderStatus status);
        List<TestApp.Models.Order> GetOrdersByDateRange(System.DateTime startDate, System.DateTime endDate);
    }
}