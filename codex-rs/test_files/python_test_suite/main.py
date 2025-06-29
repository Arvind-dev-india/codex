"""
Main application file demonstrating cross-file references and complex interactions
"""

from models import User, AdminUser, Product, DigitalProduct, Category, Order, OrderStatus
from services import UserService, OrderService
from data import UserRepository, OrderRepository, InMemoryRepository
from utils import Timer, Logger, ConfigManager, validate_email
from basic_class import BasicClass, standalone_function


def create_sample_data():
    """Create sample data for testing"""
    logger = Logger()
    config = ConfigManager()
    
    logger.log("Creating sample data...")
    
    # Create repositories
    user_repo = UserRepository()
    order_repo = OrderRepository()
    product_repo = InMemoryRepository()
    
    # Create services
    order_service = OrderService(order_repo, product_repo)
    user_service = UserService(user_repo, order_service)
    
    # Create sample users
    users = []
    user_data = [
        ("John Doe", "john@example.com"),
        ("Jane Smith", "jane@example.com"),
        ("Bob Johnson", "bob@example.com")
    ]
    
    for name, email in user_data:
        if validate_email(email):
            user = user_service.create_user(name, email)
            users.append(user)
            logger.log(f"Created user: {user.name}")
    
    # Create sample products
    products = []
    product_data = [
        ("Laptop", 999.99, "High-performance laptop"),
        ("Mouse", 29.99, "Wireless mouse"),
        ("Keyboard", 79.99, "Mechanical keyboard")
    ]
    
    for i, (name, price, description) in enumerate(product_data, 1):
        product = Product(i, name, price, description)
        product.add_stock(10)
        product_repo.add(product)
        products.append(product)
        logger.log(f"Created product: {product.name}")
    
    # Create digital product
    digital_product = DigitalProduct(4, "Software License", 199.99, "https://download.example.com/software")
    product_repo.add(digital_product)
    products.append(digital_product)
    
    # Create sample orders
    for user in users[:2]:  # Only first 2 users get orders
        order = order_service.create_order(user.id)
        
        # Add products to order
        order_service.add_product_to_order(order.id, products[0].id, 1)  # Laptop
        order_service.add_product_to_order(order.id, products[1].id, 2)  # Mouse x2
        
        logger.log(f"Created order {order.id} for user {user.name}")
    
    # Promote one user to admin
    if users:
        admin = user_service.promote_to_admin(users[0].id, admin_level=2)
        if admin:
            admin.add_permission("manage_users")
            admin.add_permission("manage_orders")
            logger.log(f"Promoted {admin.name} to admin")
    
    return {
        'users': users,
        'products': products,
        'user_service': user_service,
        'order_service': order_service,
        'logger': logger
    }


def demonstrate_functionality():
    """Demonstrate various functionality"""
    with Timer("Sample data creation"):
        data = create_sample_data()
    
    logger = data['logger']
    user_service = data['user_service']
    order_service = data['order_service']
    
    # Test basic class functionality
    basic = BasicClass(42, "test")
    result = basic.add(10, 20)
    logger.log(f"BasicClass add result: {result}")
    
    # Test standalone function
    distance = standalone_function(3.0, 4.0)
    logger.log(f"Distance calculation: {distance}")
    
    # Test user search
    search_results = user_service.search_users("john")
    logger.log(f"Search results for 'john': {len(search_results)} users found")
    
    # Test order operations
    pending_orders = order_service.get_orders_by_status(OrderStatus.PENDING)
    logger.log(f"Pending orders: {len(pending_orders)}")
    
    # Display all logs
    print("\n=== Application Logs ===")
    for log_entry in logger.get_logs():
        print(log_entry)


class ApplicationManager:
    """Main application manager class"""
    
    def __init__(self):
        self.config = ConfigManager()
        self.logger = Logger()
        self.is_running = False
    
    def start(self):
        """Start the application"""
        self.logger.log("Starting application...")
        self.is_running = True
        
        try:
            demonstrate_functionality()
        except Exception as e:
            self.logger.log(f"Application error: {e}", "ERROR")
        finally:
            self.stop()
    
    def stop(self):
        """Stop the application"""
        self.logger.log("Stopping application...")
        self.is_running = False
    
    def get_status(self) -> dict:
        """Get application status"""
        return {
            'running': self.is_running,
            'config': self.config.get_all(),
            'log_count': len(self.logger.get_logs())
        }


def main():
    """Main entry point"""
    app = ApplicationManager()
    app.start()
    
    status = app.get_status()
    print(f"\nApplication Status: {status}")


if __name__ == "__main__":
    main()