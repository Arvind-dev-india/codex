"""
User model for the Python test suite
"""

from datetime import datetime
from typing import List, Optional
from .order import Order


class User:
    """User class representing a user in the system"""
    
    def __init__(self, user_id: int = None, name: str = None, email: str = None):
        """Initialize a new user"""
        self.id = user_id
        self.name = name
        self.email = email
        self.created_at = datetime.now()
        self.orders: List[Order] = []
    
    def add_order(self, order: 'Order'):
        """Add an order to this user"""
        if order is None:
            raise ValueError("Order cannot be None")
        
        order.user_id = self.id
        self.orders.append(order)
        self._log_activity(f"Order {order.id} added to user {self.name}")
    
    def get_order(self, order_id: int) -> Optional[Order]:
        """Get an order by ID"""
        for order in self.orders:
            if order.id == order_id:
                return order
        return None
    
    def get_total_order_value(self) -> float:
        """Calculate total value of all orders"""
        total = 0.0
        for order in self.orders:
            total += order.calculate_total()
        return total
    
    def _log_activity(self, message: str):
        """Private method to log user activity"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] User Activity: {message}")
    
    def __str__(self) -> str:
        """String representation of the user"""
        return f"User: {self.name} ({self.email}) - {len(self.orders)} orders"
    
    def __repr__(self) -> str:
        """Developer representation of the user"""
        return f"User(id={self.id}, name='{self.name}', email='{self.email}')"


class AdminUser(User):
    """Admin user with additional privileges"""
    
    def __init__(self, user_id: int, name: str, email: str, admin_level: int = 1):
        """Initialize admin user"""
        super().__init__(user_id, name, email)
        self.admin_level = admin_level
        self.permissions = []
    
    def add_permission(self, permission: str):
        """Add a permission to this admin user"""
        if permission not in self.permissions:
            self.permissions.append(permission)
            self._log_activity(f"Permission '{permission}' added")
    
    def has_permission(self, permission: str) -> bool:
        """Check if admin has a specific permission"""
        return permission in self.permissions
    
    def promote_user(self, user: User) -> 'AdminUser':
        """Promote a regular user to admin"""
        admin = AdminUser(user.id, user.name, user.email)
        admin.orders = user.orders
        admin.created_at = user.created_at
        return admin