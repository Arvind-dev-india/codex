"""
Product model for the Python test suite
"""

from typing import List, Optional
from datetime import datetime


class Category:
    """Product category class"""
    
    def __init__(self, category_id: int, name: str, description: str = ""):
        """Initialize a product category"""
        self.id = category_id
        self.name = name
        self.description = description
        self.products: List['Product'] = []
    
    def add_product(self, product: 'Product'):
        """Add a product to this category"""
        if product not in self.products:
            self.products.append(product)
            product.category = self
    
    def remove_product(self, product: 'Product'):
        """Remove a product from this category"""
        if product in self.products:
            self.products.remove(product)
            product.category = None
    
    def get_product_count(self) -> int:
        """Get number of products in this category"""
        return len(self.products)
    
    def __str__(self) -> str:
        return f"Category: {self.name} ({self.get_product_count()} products)"


class Product:
    """Product class representing an item for sale"""
    
    def __init__(self, product_id: int, name: str, price: float, description: str = ""):
        """Initialize a new product"""
        self.id = product_id
        self.name = name
        self.price = price
        self.description = description
        self.category: Optional[Category] = None
        self.stock_quantity = 0
        self.created_at = datetime.now()
        self.tags: List[str] = []
    
    def update_price(self, new_price: float):
        """Update the product price"""
        if new_price < 0:
            raise ValueError("Price cannot be negative")
        
        old_price = self.price
        self.price = new_price
        self._log_price_change(old_price, new_price)
    
    def add_stock(self, quantity: int):
        """Add stock to the product"""
        if quantity <= 0:
            raise ValueError("Quantity must be positive")
        
        self.stock_quantity += quantity
        self._log_stock_change(f"Added {quantity} units")
    
    def remove_stock(self, quantity: int) -> bool:
        """Remove stock from the product"""
        if quantity <= 0:
            raise ValueError("Quantity must be positive")
        
        if self.stock_quantity >= quantity:
            self.stock_quantity -= quantity
            self._log_stock_change(f"Removed {quantity} units")
            return True
        return False
    
    def is_in_stock(self) -> bool:
        """Check if product is in stock"""
        return self.stock_quantity > 0
    
    def add_tag(self, tag: str):
        """Add a tag to the product"""
        if tag and tag not in self.tags:
            self.tags.append(tag)
    
    def remove_tag(self, tag: str) -> bool:
        """Remove a tag from the product"""
        if tag in self.tags:
            self.tags.remove(tag)
            return True
        return False
    
    def has_tag(self, tag: str) -> bool:
        """Check if product has a specific tag"""
        return tag in self.tags
    
    def calculate_discounted_price(self, discount_percentage: float) -> float:
        """Calculate price after discount"""
        if discount_percentage < 0 or discount_percentage > 100:
            raise ValueError("Discount percentage must be between 0 and 100")
        
        discount_amount = self.price * (discount_percentage / 100.0)
        return self.price - discount_amount
    
    def _log_price_change(self, old_price: float, new_price: float):
        """Log price change"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] Product {self.name}: Price changed from ${old_price:.2f} to ${new_price:.2f}")
    
    def _log_stock_change(self, message: str):
        """Log stock change"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] Product {self.name}: {message}. Current stock: {self.stock_quantity}")
    
    def __str__(self) -> str:
        """String representation of the product"""
        return f"Product: {self.name} - ${self.price:.2f} (Stock: {self.stock_quantity})"
    
    def __repr__(self) -> str:
        """Developer representation of the product"""
        return f"Product(id={self.id}, name='{self.name}', price={self.price})"


class DigitalProduct(Product):
    """Digital product that doesn't require physical stock"""
    
    def __init__(self, product_id: int, name: str, price: float, download_url: str, description: str = ""):
        """Initialize a digital product"""
        super().__init__(product_id, name, price, description)
        self.download_url = download_url
        self.download_count = 0
        self.file_size_mb = 0.0
    
    def is_in_stock(self) -> bool:
        """Digital products are always in stock"""
        return True
    
    def add_stock(self, quantity: int):
        """Digital products don't need stock management"""
        pass
    
    def remove_stock(self, quantity: int) -> bool:
        """Digital products don't need stock management"""
        return True
    
    def record_download(self):
        """Record a download of this digital product"""
        self.download_count += 1
        self._log_download()
    
    def _log_download(self):
        """Log download activity"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] Digital Product {self.name}: Downloaded. Total downloads: {self.download_count}")