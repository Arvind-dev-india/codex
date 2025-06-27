using System.Collections.Generic;
using TestApp.Models;

namespace TestApp.Services
{
    public interface IUserService
    {
        User CreateUser(string name, string email);
        User GetUser(int id);
        List<User> GetAllUsers();
        bool UpdateUser(User user);
        bool DeleteUser(int id);
        List<User> SearchUsers(string searchTerm);
    }

    public interface IOrderService
    {
        Order CreateOrder(int userId);
        Order GetOrder(int orderId);
        List<Order> GetUserOrders(int userId);
        bool UpdateOrderStatus(int orderId, OrderStatus status);
        bool CancelOrder(int orderId);
        decimal CalculateOrderTotal(int orderId);
    }

    public interface IProductService
    {
        Product CreateProduct(string name, decimal price, int stock);
        Product GetProduct(int id);
        List<Product> GetAllProducts();
        List<Product> GetProductsByCategory(string category);
        bool UpdateProductStock(int productId, int quantity);
        bool IsProductAvailable(int productId, int quantity);
    }
}