using System;
using TestApp.Models;
using TestApp.Services;
using TestApp.Data;
using TestApp.Controllers;

namespace TestApp
{
    public class Program
    {
        private static IUserService _userService;
        private static IOrderService _orderService;
        private static IProductService _productService;
        private static UserController _userController;

        public static void Main(string[] args)
        {
            Console.WriteLine("Starting TestApp...");
            
            try
            {
                InitializeServices();
                RunDemo();
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Application error: {ex.Message}");
            }
            
            Console.WriteLine("Application finished. Press any key to exit.");
            Console.ReadKey();
        }

        private static void InitializeServices()
        {
            Console.WriteLine("Initializing services...");
            
            // Create repositories
            var userRepository = new UserRepository();
            var orderRepository = new OrderRepository();
            
            // Create services
            _productService = new ProductService();
            _orderService = new OrderService(orderRepository, _productService);
            _userService = new UserService(userRepository, _orderService);
            
            // Create controllers
            _userController = new UserController(_userService, _orderService);
            
            Console.WriteLine("Services initialized successfully.");
        }

        private static void RunDemo()
        {
            Console.WriteLine("Running application demo...");
            
            DemoUserOperations();
            DemoOrderOperations();
            DemoProductOperations();
        }

        private static void DemoUserOperations()
        {
            Console.WriteLine("\n=== User Operations Demo ===");
            
            // Create users
            var user1 = CreateDemoUser("John Doe", "john@example.com");
            var user2 = CreateDemoUser("Jane Smith", "jane@example.com");
            
            // Get all users
            var allUsers = _userService.GetAllUsers();
            Console.WriteLine($"Total users: {allUsers.Count}");
            
            // Search users
            var searchResults = _userService.SearchUsers("john");
            Console.WriteLine($"Search results for 'john': {searchResults.Count}");
            
            // Update user
            if (user1 != null)
            {
                user1.Name = "John Updated";
                _userService.UpdateUser(user1);
            }
        }

        private static void DemoOrderOperations()
        {
            Console.WriteLine("\n=== Order Operations Demo ===");
            
            var users = _userService.GetAllUsers();
            if (users.Count > 0)
            {
                var user = users[0];
                
                // Create order
                var order = _orderService.CreateOrder(user.Id);
                Console.WriteLine($"Created order {order.Id} for user {user.Name}");
                
                // Add products to order
                AddProductsToOrder(order);
                
                // Update order status
                _orderService.UpdateOrderStatus(order.Id, OrderStatus.Processing);
                _orderService.UpdateOrderStatus(order.Id, OrderStatus.Shipped);
                
                // Calculate total
                var total = _orderService.CalculateOrderTotal(order.Id);
                Console.WriteLine($"Order total: ${total}");
            }
        }

        private static void DemoProductOperations()
        {
            Console.WriteLine("\n=== Product Operations Demo ===");
            
            // This would use ProductService if it was implemented
            var product = new Product(1, "Demo Product", 29.99m, 100);
            Console.WriteLine($"Demo product: {product}");
            
            // Test stock operations
            product.ReserveStock(5);
            product.UpdateStock(10);
            
            var discountedPrice = product.GetDiscountedPrice(10);
            Console.WriteLine($"Discounted price (10% off): ${discountedPrice}");
        }

        private static User CreateDemoUser(string name, string email)
        {
            try
            {
                var user = _userService.CreateUser(name, email);
                Console.WriteLine($"Created user: {user.Name} ({user.Email})");
                return user;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Failed to create user {name}: {ex.Message}");
                return null;
            }
        }

        private static void AddProductsToOrder(Order order)
        {
            var product1 = new Product(1, "Widget A", 19.99m, 50);
            var product2 = new Product(2, "Widget B", 29.99m, 30);
            
            order.AddItem(product1, 2);
            order.AddItem(product2, 1);
            
            Console.WriteLine($"Added {order.Items.Count} items to order {order.Id}");
        }
    }

    // Placeholder ProductService for demo
    public class ProductService : IProductService
    {
        public Product CreateProduct(string name, decimal price, int stock)
        {
            return new Product(1, name, price, stock);
        }

        public Product GetProduct(int id)
        {
            return new Product(id, "Demo Product", 19.99m, 100);
        }

        public List<Product> GetAllProducts()
        {
            return new List<Product>();
        }

        public List<Product> GetProductsByCategory(string category)
        {
            return new List<Product>();
        }

        public bool UpdateProductStock(int productId, int quantity)
        {
            return true;
        }

        public bool IsProductAvailable(int productId, int quantity)
        {
            return true;
        }
    }
}