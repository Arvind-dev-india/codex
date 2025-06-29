"""
Order service for managing order operations
"""

from typing import List, Optional
from datetime import datetime
from ..models.order import Order, OrderStatus, OrderItem
from ..models.product import Product
from ..data.repository import Repository


class OrderService:
    """Service class for order management operations"""
    
    def __init__(self, order_repository: Repository, product_repository: Repository):
        """Initialize the order service"""
        if order_repository is None:
            raise ValueError("Order repository cannot be None")
        if product_repository is None:
            raise ValueError("Product repository cannot be None")
        
        self._order_repository = order_repository
        self._product_repository = product_repository
    
    def create_order(self, user_id: int) -> Order:
        """Create a new order for a user"""
        if user_id <= 0:
            raise ValueError("User ID must be positive")
        
        order = Order(
            order_id=self._generate_order_id(),
            user_id=user_id
        )
        
        self._order_repository.add(order)
        self._log_order_creation(order)
        return order
    
    def get_order(self, order_id: int) -> Optional[Order]:
        """Get an order by ID"""
        return self._order_repository.get_by_id(order_id)
    
    def get_user_orders(self, user_id: int) -> List[Order]:
        """Get all orders for a specific user"""
        all_orders = self._order_repository.get_all()
        return [order for order in all_orders if order.user_id == user_id]
    
    def add_product_to_order(self, order_id: int, product_id: int, quantity: int = 1) -> bool:
        """Add a product to an order"""
        order = self.get_order(order_id)
        if order is None:
            return False
        
        if order.status != OrderStatus.PENDING:
            raise ValueError("Cannot modify order that is not pending")
        
        product = self._product_repository.get_by_id(product_id)
        if product is None:
            raise ValueError(f"Product with ID {product_id} not found")
        
        if not product.is_in_stock():
            raise ValueError(f"Product {product.name} is out of stock")
        
        if not product.remove_stock(quantity):
            raise ValueError(f"Insufficient stock for product {product.name}")
        
        order.add_item(product, quantity)
        self._order_repository.update(order)
        self._log_product_added(order, product, quantity)
        return True
    
    def remove_product_from_order(self, order_id: int, product_id: int) -> bool:
        """Remove a product from an order"""
        order = self.get_order(order_id)
        if order is None:
            return False
        
        if order.status != OrderStatus.PENDING:
            raise ValueError("Cannot modify order that is not pending")
        
        # Find the item to restore stock
        item_to_remove = None
        for item in order.items:
            if item.product.id == product_id:
                item_to_remove = item
                break
        
        if item_to_remove:
            # Restore stock
            product = self._product_repository.get_by_id(product_id)
            if product:
                product.add_stock(item_to_remove.quantity)
                self._product_repository.update(product)
        
        success = order.remove_item(product_id)
        if success:
            self._order_repository.update(order)
            self._log_product_removed(order, product_id)
        
        return success
    
    def update_order_status(self, order_id: int, new_status: OrderStatus) -> bool:
        """Update the status of an order"""
        order = self.get_order(order_id)
        if order is None:
            return False
        
        self._validate_status_transition(order.status, new_status)
        order.update_status(new_status)
        self._order_repository.update(order)
        return True
    
    def cancel_order(self, order_id: int) -> bool:
        """Cancel an order and restore stock"""
        order = self.get_order(order_id)
        if order is None:
            return False
        
        if order.status in [OrderStatus.DELIVERED, OrderStatus.CANCELLED]:
            raise ValueError("Cannot cancel order that is already delivered or cancelled")
        
        # Restore stock for all items
        for item in order.items:
            product = self._product_repository.get_by_id(item.product.id)
            if product:
                product.add_stock(item.quantity)
                self._product_repository.update(product)
        
        order.update_status(OrderStatus.CANCELLED)
        self._order_repository.update(order)
        self._log_order_cancellation(order)
        return True
    
    def calculate_order_summary(self, order_id: int) -> Optional[dict]:
        """Calculate order summary with totals"""
        order = self.get_order(order_id)
        if order is None:
            return None
        
        return {
            'order_id': order.id,
            'status': order.status.value,
            'item_count': len(order.items),
            'subtotal': order.calculate_subtotal(),
            'discount': order.calculate_discount(),
            'tax': order.calculate_tax(),
            'total': order.calculate_total(),
            'created_at': order.created_at.isoformat()
        }
    
    def get_orders_by_status(self, status: OrderStatus) -> List[Order]:
        """Get all orders with a specific status"""
        all_orders = self._order_repository.get_all()
        return [order for order in all_orders if order.status == status]
    
    def get_orders_by_date_range(self, start_date: datetime, end_date: datetime) -> List[Order]:
        """Get orders within a date range"""
        all_orders = self._order_repository.get_all()
        return [
            order for order in all_orders 
            if start_date <= order.created_at <= end_date
        ]
    
    def _generate_order_id(self) -> int:
        """Generate a new order ID"""
        orders = self._order_repository.get_all()
        if not orders:
            return 1
        return max(order.id for order in orders) + 1
    
    def _validate_status_transition(self, current_status: OrderStatus, new_status: OrderStatus):
        """Validate if status transition is allowed"""
        valid_transitions = {
            OrderStatus.PENDING: [OrderStatus.PROCESSING, OrderStatus.CANCELLED],
            OrderStatus.PROCESSING: [OrderStatus.SHIPPED, OrderStatus.CANCELLED],
            OrderStatus.SHIPPED: [OrderStatus.DELIVERED],
            OrderStatus.DELIVERED: [],
            OrderStatus.CANCELLED: []
        }
        
        if new_status not in valid_transitions.get(current_status, []):
            raise ValueError(f"Invalid status transition from {current_status.value} to {new_status.value}")
    
    def _log_order_creation(self, order: Order):
        """Log order creation"""
        print(f"Order created: {order.id} for user {order.user_id}")
    
    def _log_product_added(self, order: Order, product: Product, quantity: int):
        """Log product addition to order"""
        print(f"Added {quantity}x {product.name} to order {order.id}")
    
    def _log_product_removed(self, order: Order, product_id: int):
        """Log product removal from order"""
        print(f"Removed product {product_id} from order {order.id}")
    
    def _log_order_cancellation(self, order: Order):
        """Log order cancellation"""
        print(f"Order {order.id} cancelled, stock restored")