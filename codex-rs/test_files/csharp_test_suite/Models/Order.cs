using System;
using System.Collections.Generic;
using System.Linq;

namespace TestApp.Models
{
    public enum OrderStatus
    {
        Pending,
        Processing,
        Shipped,
        Delivered,
        Cancelled
    }

    public class Order
    {
        public int Id { get; set; }
        public int UserId { get; set; }
        public DateTime OrderDate { get; set; }
        public OrderStatus Status { get; set; }
        public List<OrderItem> Items { get; set; }

        public Order()
        {
            Items = new List<OrderItem>();
            OrderDate = DateTime.Now;
            Status = OrderStatus.Pending;
        }

        public Order(int id, int userId) : this()
        {
            Id = id;
            UserId = userId;
        }

        public void AddItem(Product product, int quantity)
        {
            if (product == null)
                throw new ArgumentNullException(nameof(product));

            var existingItem = FindItem(product.Id);
            if (existingItem != null)
            {
                existingItem.UpdateQuantity(existingItem.Quantity + quantity);
            }
            else
            {
                var newItem = new OrderItem(product, quantity);
                Items.Add(newItem);
            }
            
            UpdateStatus();
        }

        public void RemoveItem(int productId)
        {
            var item = FindItem(productId);
            if (item != null)
            {
                Items.Remove(item);
                UpdateStatus();
            }
        }

        private OrderItem FindItem(int productId)
        {
            return Items.FirstOrDefault(i => i.Product.Id == productId);
        }

        public decimal CalculateTotal()
        {
            decimal total = 0;
            foreach (var item in Items)
            {
                total += item.GetSubtotal();
            }
            return total;
        }

        public void UpdateStatus()
        {
            if (Items.Count == 0)
            {
                Status = OrderStatus.Cancelled;
            }
            else if (Status == OrderStatus.Pending)
            {
                Status = OrderStatus.Processing;
            }
        }

        public bool CanShip()
        {
            return Status == OrderStatus.Processing && Items.All(i => i.Product.IsInStock());
        }

        public void Ship()
        {
            if (CanShip())
            {
                Status = OrderStatus.Shipped;
                NotifyShipping();
            }
        }

        private void NotifyShipping()
        {
            Console.WriteLine($"Order {Id} has been shipped");
        }
    }
}