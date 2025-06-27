using System;

namespace TestApp.Models
{
    public class Product
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Description { get; set; }
        public decimal Price { get; set; }
        public int StockQuantity { get; set; }
        public string Category { get; set; }

        public Product()
        {
        }

        public Product(int id, string name, decimal price, int stockQuantity)
        {
            Id = id;
            Name = name;
            Price = price;
            StockQuantity = stockQuantity;
        }

        public bool IsInStock()
        {
            return StockQuantity > 0;
        }

        public bool IsInStock(int requiredQuantity)
        {
            return StockQuantity >= requiredQuantity;
        }

        public void UpdateStock(int quantity)
        {
            StockQuantity += quantity;
            if (StockQuantity < 0)
            {
                StockQuantity = 0;
            }
            LogStockChange(quantity);
        }

        public void ReserveStock(int quantity)
        {
            if (!IsInStock(quantity))
            {
                throw new InvalidOperationException($"Insufficient stock for product {Name}");
            }
            UpdateStock(-quantity);
        }

        public decimal GetDiscountedPrice(decimal discountPercentage)
        {
            if (discountPercentage < 0 || discountPercentage > 100)
            {
                throw new ArgumentException("Discount percentage must be between 0 and 100");
            }
            
            return Price * (1 - discountPercentage / 100);
        }

        private void LogStockChange(int change)
        {
            string action = change > 0 ? "increased" : "decreased";
            Console.WriteLine($"Stock for {Name} {action} by {Math.Abs(change)}. Current stock: {StockQuantity}");
        }

        public override string ToString()
        {
            return $"{Name} - ${Price} (Stock: {StockQuantity})";
        }
    }

    public class OrderItem
    {
        public Product Product { get; set; }
        public int Quantity { get; set; }

        public OrderItem(Product product, int quantity)
        {
            Product = product ?? throw new ArgumentNullException(nameof(product));
            Quantity = quantity;
        }

        public decimal GetSubtotal()
        {
            return Product.Price * Quantity;
        }

        public void UpdateQuantity(int newQuantity)
        {
            if (newQuantity < 0)
                throw new ArgumentException("Quantity cannot be negative");
            
            Quantity = newQuantity;
        }
    }
}