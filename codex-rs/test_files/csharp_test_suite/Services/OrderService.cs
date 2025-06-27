using System;
using System.Collections.Generic;
using System.Linq;
using TestApp.Models;
using TestApp.Data;

namespace TestApp.Services
{
    public class OrderService : IOrderService
    {
        private readonly IRepository<Order> _orderRepository;
        private readonly IProductService _productService;

        public OrderService(IRepository<Order> orderRepository, IProductService productService)
        {
            _orderRepository = orderRepository ?? throw new ArgumentNullException(nameof(orderRepository));
            _productService = productService ?? throw new ArgumentNullException(nameof(productService));
        }

        public Order CreateOrder(int userId)
        {
            if (userId <= 0)
                throw new ArgumentException("Invalid user ID", nameof(userId));

            var order = new Order
            {
                Id = GenerateOrderId(),
                UserId = userId
            };

            _orderRepository.Add(order);
            LogOrderCreation(order);
            return order;
        }

        public Order GetOrder(int orderId)
        {
            var order = _orderRepository.GetById(orderId);
            if (order != null)
            {
                LoadOrderItems(order);
            }
            return order;
        }

        public List<Order> GetUserOrders(int userId)
        {
            var allOrders = _orderRepository.GetAll();
            var userOrders = allOrders.Where(o => o.UserId == userId).ToList();
            
            foreach (var order in userOrders)
            {
                LoadOrderItems(order);
            }
            
            return userOrders;
        }

        public bool UpdateOrderStatus(int orderId, OrderStatus status)
        {
            var order = GetOrder(orderId);
            if (order == null)
                return false;

            if (!IsValidStatusTransition(order.Status, status))
                throw new InvalidOperationException($"Cannot change status from {order.Status} to {status}");

            order.Status = status;
            _orderRepository.Update(order);
            LogStatusUpdate(order, status);
            
            if (status == OrderStatus.Shipped)
            {
                ProcessShipping(order);
            }
            
            return true;
        }

        public bool CancelOrder(int orderId)
        {
            var order = GetOrder(orderId);
            if (order == null)
                return false;

            if (order.Status == OrderStatus.Shipped || order.Status == OrderStatus.Delivered)
                throw new InvalidOperationException("Cannot cancel shipped or delivered orders");

            RestoreProductStock(order);
            order.Status = OrderStatus.Cancelled;
            _orderRepository.Update(order);
            LogOrderCancellation(order);
            return true;
        }

        public decimal CalculateOrderTotal(int orderId)
        {
            var order = GetOrder(orderId);
            if (order == null)
                throw new ArgumentException("Order not found", nameof(orderId));

            return order.CalculateTotal();
        }

        private int GenerateOrderId()
        {
            var orders = _orderRepository.GetAll();
            return orders.Count > 0 ? orders.Max(o => o.Id) + 1 : 1;
        }

        private void LoadOrderItems(Order order)
        {
            // In a real application, this would load from a separate OrderItems table
            // For this test, we'll assume items are already loaded
        }

        private bool IsValidStatusTransition(OrderStatus currentStatus, OrderStatus newStatus)
        {
            return currentStatus switch
            {
                OrderStatus.Pending => newStatus == OrderStatus.Processing || newStatus == OrderStatus.Cancelled,
                OrderStatus.Processing => newStatus == OrderStatus.Shipped || newStatus == OrderStatus.Cancelled,
                OrderStatus.Shipped => newStatus == OrderStatus.Delivered,
                _ => false
            };
        }

        private void ProcessShipping(Order order)
        {
            ReserveProductStock(order);
            NotifyShipping(order);
        }

        private void ReserveProductStock(Order order)
        {
            foreach (var item in order.Items)
            {
                var product = _productService.GetProduct(item.Product.Id);
                if (product != null)
                {
                    product.ReserveStock(item.Quantity);
                }
            }
        }

        private void RestoreProductStock(Order order)
        {
            foreach (var item in order.Items)
            {
                _productService.UpdateProductStock(item.Product.Id, item.Quantity);
            }
        }

        private void NotifyShipping(Order order)
        {
            Console.WriteLine($"Shipping notification sent for order {order.Id}");
        }

        private void LogOrderCreation(Order order)
        {
            Console.WriteLine($"Order created: {order.Id} for user {order.UserId}");
        }

        private void LogStatusUpdate(Order order, OrderStatus newStatus)
        {
            Console.WriteLine($"Order {order.Id} status updated to {newStatus}");
        }

        private void LogOrderCancellation(Order order)
        {
            Console.WriteLine($"Order {order.Id} cancelled");
        }
    }
}