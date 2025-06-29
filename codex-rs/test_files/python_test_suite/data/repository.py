"""
Repository pattern implementation for data access
"""

from typing import List, Optional, TypeVar, Generic, Dict
from abc import ABC, abstractmethod

T = TypeVar('T')


class Repository(ABC, Generic[T]):
    """Abstract base repository class"""
    
    @abstractmethod
    def add(self, entity: T) -> T:
        """Add an entity to the repository"""
        pass
    
    @abstractmethod
    def get_by_id(self, entity_id: int) -> Optional[T]:
        """Get an entity by its ID"""
        pass
    
    @abstractmethod
    def get_all(self) -> List[T]:
        """Get all entities"""
        pass
    
    @abstractmethod
    def update(self, entity: T) -> bool:
        """Update an entity"""
        pass
    
    @abstractmethod
    def delete(self, entity_id: int) -> bool:
        """Delete an entity by ID"""
        pass


class InMemoryRepository(Repository[T]):
    """In-memory implementation of the repository pattern"""
    
    def __init__(self):
        """Initialize the in-memory repository"""
        self._data: Dict[int, T] = {}
        self._next_id = 1
    
    def add(self, entity: T) -> T:
        """Add an entity to the repository"""
        if entity is None:
            raise ValueError("Entity cannot be None")
        
        # Assign ID if entity doesn't have one
        if not hasattr(entity, 'id') or entity.id is None:
            entity.id = self._next_id
            self._next_id += 1
        
        self._data[entity.id] = entity
        self._log_operation("ADD", entity.id)
        return entity
    
    def get_by_id(self, entity_id: int) -> Optional[T]:
        """Get an entity by its ID"""
        if entity_id is None:
            return None
        
        entity = self._data.get(entity_id)
        if entity:
            self._log_operation("GET", entity_id)
        return entity
    
    def get_all(self) -> List[T]:
        """Get all entities"""
        entities = list(self._data.values())
        self._log_operation("GET_ALL", len(entities))
        return entities
    
    def update(self, entity: T) -> bool:
        """Update an entity"""
        if entity is None:
            raise ValueError("Entity cannot be None")
        
        if not hasattr(entity, 'id') or entity.id is None:
            return False
        
        if entity.id in self._data:
            self._data[entity.id] = entity
            self._log_operation("UPDATE", entity.id)
            return True
        
        return False
    
    def delete(self, entity_id: int) -> bool:
        """Delete an entity by ID"""
        if entity_id in self._data:
            del self._data[entity_id]
            self._log_operation("DELETE", entity_id)
            return True
        
        return False
    
    def count(self) -> int:
        """Get the count of entities"""
        return len(self._data)
    
    def clear(self):
        """Clear all entities"""
        count = len(self._data)
        self._data.clear()
        self._log_operation("CLEAR", count)
    
    def exists(self, entity_id: int) -> bool:
        """Check if an entity exists"""
        return entity_id in self._data
    
    def find_by_predicate(self, predicate) -> List[T]:
        """Find entities matching a predicate function"""
        matching_entities = []
        for entity in self._data.values():
            if predicate(entity):
                matching_entities.append(entity)
        
        self._log_operation("FIND", len(matching_entities))
        return matching_entities
    
    def _log_operation(self, operation: str, identifier):
        """Log repository operations"""
        print(f"Repository {operation}: {identifier}")


class UserRepository(InMemoryRepository):
    """Specialized repository for User entities"""
    
    def find_by_email(self, email: str) -> Optional[T]:
        """Find a user by email address"""
        for user in self._data.values():
            if hasattr(user, 'email') and user.email.lower() == email.lower():
                self._log_operation("FIND_BY_EMAIL", email)
                return user
        return None
    
    def find_by_name(self, name: str) -> List[T]:
        """Find users by name (partial match)"""
        matching_users = []
        name_lower = name.lower()
        
        for user in self._data.values():
            if hasattr(user, 'name') and name_lower in user.name.lower():
                matching_users.append(user)
        
        self._log_operation("FIND_BY_NAME", f"{name} ({len(matching_users)} found)")
        return matching_users


class OrderRepository(InMemoryRepository):
    """Specialized repository for Order entities"""
    
    def find_by_user_id(self, user_id: int) -> List[T]:
        """Find orders by user ID"""
        matching_orders = []
        
        for order in self._data.values():
            if hasattr(order, 'user_id') and order.user_id == user_id:
                matching_orders.append(order)
        
        self._log_operation("FIND_BY_USER_ID", f"User {user_id} ({len(matching_orders)} orders)")
        return matching_orders
    
    def find_by_status(self, status) -> List[T]:
        """Find orders by status"""
        matching_orders = []
        
        for order in self._data.values():
            if hasattr(order, 'status') and order.status == status:
                matching_orders.append(order)
        
        self._log_operation("FIND_BY_STATUS", f"{status} ({len(matching_orders)} orders)")
        return matching_orders