"""
Utility functions and helper classes
"""

import re
import hashlib
from typing import List, Dict, Any, Optional
from datetime import datetime, timedelta
from functools import wraps


def validate_email(email: str) -> bool:
    """Validate email address format"""
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None


def generate_hash(text: str) -> str:
    """Generate SHA-256 hash of text"""
    return hashlib.sha256(text.encode()).hexdigest()


def format_currency(amount: float, currency: str = "USD") -> str:
    """Format amount as currency"""
    if currency == "USD":
        return f"${amount:.2f}"
    elif currency == "EUR":
        return f"€{amount:.2f}"
    else:
        return f"{amount:.2f} {currency}"


def calculate_percentage(part: float, whole: float) -> float:
    """Calculate percentage"""
    if whole == 0:
        return 0.0
    return (part / whole) * 100.0


def retry_on_failure(max_attempts: int = 3, delay: float = 1.0):
    """Decorator to retry function on failure"""
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            last_exception = None
            
            for attempt in range(max_attempts):
                try:
                    return func(*args, **kwargs)
                except Exception as e:
                    last_exception = e
                    if attempt < max_attempts - 1:
                        print(f"Attempt {attempt + 1} failed: {e}. Retrying in {delay}s...")
                        import time
                        time.sleep(delay)
                    else:
                        print(f"All {max_attempts} attempts failed.")
            
            raise last_exception
        return wrapper
    return decorator


class Timer:
    """Context manager for timing operations"""
    
    def __init__(self, name: str = "Operation"):
        self.name = name
        self.start_time = None
        self.end_time = None
    
    def __enter__(self):
        self.start_time = datetime.now()
        print(f"Starting {self.name}...")
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.end_time = datetime.now()
        duration = self.end_time - self.start_time
        print(f"{self.name} completed in {duration.total_seconds():.3f} seconds")
    
    def elapsed(self) -> timedelta:
        """Get elapsed time"""
        if self.start_time is None:
            return timedelta(0)
        
        end = self.end_time or datetime.now()
        return end - self.start_time


class DataValidator:
    """Class for validating data structures"""
    
    @staticmethod
    def validate_user_data(data: Dict[str, Any]) -> List[str]:
        """Validate user data and return list of errors"""
        errors = []
        
        if not data.get('name'):
            errors.append("Name is required")
        elif len(data['name']) < 2:
            errors.append("Name must be at least 2 characters")
        
        if not data.get('email'):
            errors.append("Email is required")
        elif not validate_email(data['email']):
            errors.append("Invalid email format")
        
        return errors
    
    @staticmethod
    def validate_product_data(data: Dict[str, Any]) -> List[str]:
        """Validate product data and return list of errors"""
        errors = []
        
        if not data.get('name'):
            errors.append("Product name is required")
        
        price = data.get('price')
        if price is None:
            errors.append("Price is required")
        elif not isinstance(price, (int, float)) or price < 0:
            errors.append("Price must be a non-negative number")
        
        stock = data.get('stock_quantity')
        if stock is not None and (not isinstance(stock, int) or stock < 0):
            errors.append("Stock quantity must be a non-negative integer")
        
        return errors


class ConfigManager:
    """Simple configuration manager"""
    
    def __init__(self):
        self._config: Dict[str, Any] = {}
        self._load_defaults()
    
    def _load_defaults(self):
        """Load default configuration values"""
        self._config = {
            'app_name': 'Python Test Suite',
            'version': '1.0.0',
            'debug': False,
            'max_items_per_page': 50,
            'default_currency': 'USD',
            'tax_rate': 0.08,
            'discount_threshold': 100.0
        }
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get configuration value"""
        return self._config.get(key, default)
    
    def set(self, key: str, value: Any):
        """Set configuration value"""
        self._config[key] = value
    
    def update(self, config_dict: Dict[str, Any]):
        """Update multiple configuration values"""
        self._config.update(config_dict)
    
    def get_all(self) -> Dict[str, Any]:
        """Get all configuration values"""
        return self._config.copy()


def batch_process(items: List[Any], batch_size: int = 10):
    """Generator to process items in batches"""
    for i in range(0, len(items), batch_size):
        yield items[i:i + batch_size]


def deep_merge(dict1: Dict, dict2: Dict) -> Dict:
    """Deep merge two dictionaries"""
    result = dict1.copy()
    
    for key, value in dict2.items():
        if key in result and isinstance(result[key], dict) and isinstance(value, dict):
            result[key] = deep_merge(result[key], value)
        else:
            result[key] = value
    
    return result


class Singleton:
    """Singleton metaclass"""
    _instances = {}
    
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class Logger(metaclass=Singleton):
    """Simple singleton logger"""
    
    def __init__(self):
        self.logs: List[str] = []
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        log_entry = f"[{timestamp}] {level}: {message}"
        self.logs.append(log_entry)
        print(log_entry)
    
    def get_logs(self) -> List[str]:
        """Get all logs"""
        return self.logs.copy()
    
    def clear_logs(self):
        """Clear all logs"""
        self.logs.clear()