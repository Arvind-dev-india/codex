"""
User service for managing user operations
"""

from typing import List, Optional
from ..models.user import User, AdminUser
from ..models.order import Order
from ..data.repository import Repository
from .order_service import OrderService


class UserService:
    """Service class for user management operations"""
    
    def __init__(self, user_repository: Repository, order_service: OrderService):
        """Initialize the user service"""
        if user_repository is None:
            raise ValueError("User repository cannot be None")
        if order_service is None:
            raise ValueError("Order service cannot be None")
        
        self._user_repository = user_repository
        self._order_service = order_service
    
    def create_user(self, name: str, email: str) -> User:
        """Create a new user"""
        if not name or not name.strip():
            raise ValueError("Name cannot be empty")
        
        if not email or not email.strip():
            raise ValueError("Email cannot be empty")
        
        if self._is_email_exists(email):
            raise ValueError(f"User with email {email} already exists")
        
        user = User(
            user_id=self._generate_user_id(),
            name=name.strip(),
            email=email.strip()
        )
        
        self._user_repository.add(user)
        self._log_user_creation(user)
        return user
    
    def get_user(self, user_id: int) -> Optional[User]:
        """Get a user by ID"""
        user = self._user_repository.get_by_id(user_id)
        if user:
            self._load_user_orders(user)
        return user
    
    def get_all_users(self) -> List[User]:
        """Get all users"""
        users = self._user_repository.get_all()
        for user in users:
            self._load_user_orders(user)
        return users
    
    def update_user(self, user: User) -> bool:
        """Update an existing user"""
        if user is None:
            raise ValueError("User cannot be None")
        
        existing_user = self._user_repository.get_by_id(user.id)
        if existing_user is None:
            return False
        
        self._validate_user_update(user, existing_user)
        self._user_repository.update(user)
        self._log_user_update(user)
        return True
    
    def delete_user(self, user_id: int) -> bool:
        """Delete a user"""
        user = self._user_repository.get_by_id(user_id)
        if user is None:
            return False
        
        if self._has_active_orders(user):
            raise ValueError("Cannot delete user with active orders")
        
        self._user_repository.delete(user_id)
        self._log_user_deletion(user)
        return True
    
    def search_users(self, search_term: str) -> List[User]:
        """Search users by name or email"""
        if not search_term or not search_term.strip():
            return []
        
        all_users = self.get_all_users()
        search_term = search_term.lower()
        
        matching_users = []
        for user in all_users:
            if (search_term in user.name.lower() or 
                search_term in user.email.lower()):
                matching_users.append(user)
        
        return matching_users
    
    def promote_to_admin(self, user_id: int, admin_level: int = 1) -> Optional[AdminUser]:
        """Promote a regular user to admin"""
        user = self.get_user(user_id)
        if user is None:
            return None
        
        if isinstance(user, AdminUser):
            user.admin_level = admin_level
            self.update_user(user)
            return user
        
        admin_user = AdminUser(user.id, user.name, user.email, admin_level)
        admin_user.orders = user.orders
        admin_user.created_at = user.created_at
        
        self._user_repository.update(admin_user)
        self._log_user_promotion(admin_user)
        return admin_user
    
    def _is_email_exists(self, email: str) -> bool:
        """Check if email already exists"""
        users = self._user_repository.get_all()
        return any(user.email.lower() == email.lower() for user in users)
    
    def _generate_user_id(self) -> int:
        """Generate a new user ID"""
        users = self._user_repository.get_all()
        if not users:
            return 1
        return max(user.id for user in users) + 1
    
    def _load_user_orders(self, user: User):
        """Load orders for a user"""
        orders = self._order_service.get_user_orders(user.id)
        user.orders = orders
    
    def _validate_user_update(self, new_user: User, existing_user: User):
        """Validate user update"""
        if (new_user.email != existing_user.email and 
            self._is_email_exists(new_user.email)):
            raise ValueError(f"Email {new_user.email} is already in use")
    
    def _has_active_orders(self, user: User) -> bool:
        """Check if user has active orders"""
        orders = self._order_service.get_user_orders(user.id)
        from ..models.order import OrderStatus
        
        active_statuses = {OrderStatus.PENDING, OrderStatus.PROCESSING, OrderStatus.SHIPPED}
        return any(order.status in active_statuses for order in orders)
    
    def _log_user_creation(self, user: User):
        """Log user creation"""
        print(f"User created: {user.name} ({user.email})")
    
    def _log_user_update(self, user: User):
        """Log user update"""
        print(f"User updated: {user.name} ({user.email})")
    
    def _log_user_deletion(self, user: User):
        """Log user deletion"""
        print(f"User deleted: {user.name} ({user.email})")
    
    def _log_user_promotion(self, admin_user: AdminUser):
        """Log user promotion to admin"""
        print(f"User promoted to admin: {admin_user.name} (Level {admin_user.admin_level})")