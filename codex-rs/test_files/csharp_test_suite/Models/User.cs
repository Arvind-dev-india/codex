using System;
using System.Collections.Generic;

namespace TestApp.Models
{
    public class User
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Email { get; set; }
        public DateTime CreatedAt { get; set; }
        public List<Order> Orders { get; set; }

        public User()
        {
            Orders = new List<Order>();
            CreatedAt = DateTime.Now;
        }

        public User(int id, string name, string email) : this()
        {
            Id = id;
            Name = name;
            Email = email;
        }

        public void AddOrder(Order order)
        {
            if (order == null)
                throw new ArgumentNullException(nameof(order));
            
            order.UserId = Id;
            Orders.Add(order);
            LogActivity($"Order {order.Id} added to user {Name}");
        }

        public Order GetOrder(int orderId)
        {
            return Orders.Find(o => o.Id == orderId);
        }

        public decimal GetTotalOrderValue()
        {
            decimal total = 0;
            foreach (var order in Orders)
            {
                total += order.CalculateTotal();
            }
            return total;
        }

        private void LogActivity(string message)
        {
            Console.WriteLine($"[{DateTime.Now}] User Activity: {message}");
        }

        public override string ToString()
        {
            return $"User: {Name} ({Email}) - {Orders.Count} orders";
        }
    }
}