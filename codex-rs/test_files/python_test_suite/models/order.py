"""
Order model for the Python test suite
"""

from datetime import datetime
from enum import Enum
from typing import List, Optional
from .product import Product


class OrderStatus(Enum):
    """Enumeration for order status"""
    PENDING = "pending"
    PROCESSING = "processing"
    SHIPPED = "shipped"
    DELIVERED = "delivered"
    CANCELLED = "cancelled"


class Order:
    """Order class representing a customer order"""
    
    def __init__(self, order_id: int, user_id: int = None):
        """Initialize a new order"""
        self.id = order_id
        self.user_id = user_id
        self.status = OrderStatus.PENDING
        self.created_at = datetime.now()
        self.items: List[OrderItem] = []
        self.discount_percentage = 0.0
        self.tax_rate = 0.08
    
    def add_item(self, product: Product, quantity: int = 1):
        """Add a product to this order"""
        if product is None:
            raise ValueError("Product cannot be None")
        if quantity <= 0:
            raise ValueError("Quantity must be positive")
        
        # Check if product already exists in order
        existing_item = self._find_item_by_product(product.id)
        if existing_item:
            existing_item.quantity += quantity
        else:
            item = OrderItem(product, quantity)
            self.items.append(item)
    
    def remove_item(self, product_id: int) -> bool:
        """Remove a product from this order"""
        for i, item in enumerate(self.items):
            if item.product.id == product_id:
                del self.items[i]
                return True
        return False
    
    def calculate_subtotal(self) -> float:
        """Calculate subtotal before tax and discount"""
        subtotal = 0.0
        for item in self.items:
            subtotal += item.get_total_price()
        return subtotal
    
    def calculate_discount(self) -> float:
        """Calculate discount amount"""
        subtotal = self.calculate_subtotal()
        return subtotal * (self.discount_percentage / 100.0)
    
    def calculate_tax(self) -> float:
        """Calculate tax amount"""
        subtotal = self.calculate_subtotal()
        discount = self.calculate_discount()
        return (subtotal - discount) * self.tax_rate
    
    def calculate_total(self) -> float:
        """Calculate total order amount"""
        subtotal = self.calculate_subtotal()
        discount = self.calculate_discount()
        tax = self.calculate_tax()
        return subtotal - discount + tax
    
    def update_status(self, new_status: OrderStatus):
        """Update the order status"""
        if not isinstance(new_status, OrderStatus):
            raise ValueError("Status must be an OrderStatus enum value")
        
        old_status = self.status
        self.status = new_status
        self._log_status_change(old_status, new_status)
    
    def _find_item_by_product(self, product_id: int) -> Optional['OrderItem']:
        """Find an order item by product ID"""
        for item in self.items:
            if item.product.id == product_id:
                return item
        return None
    
    def _log_status_change(self, old_status: OrderStatus, new_status: OrderStatus):
        """Log status change"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] Order {self.id}: {old_status.value} -> {new_status.value}")
    
    def __str__(self) -> str:
        """String representation of the order"""
        return f"Order {self.id}: {len(self.items)} items, Status: {self.status.value}"


class OrderItem:
    """Individual item within an order"""
    
    def __init__(self, product: Product, quantity: int):
        """Initialize an order item"""
        self.product = product
        self.quantity = quantity
    
    def get_total_price(self) -> float:
        """Get total price for this item"""
        return self.product.price * self.quantity
    
    def __str__(self) -> str:
        """String representation of the order item"""
        return f"{self.quantity}x {self.product.name} @ ${self.product.price}"