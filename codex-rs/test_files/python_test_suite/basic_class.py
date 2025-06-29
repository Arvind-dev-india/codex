"""
A basic Python module for testing code analysis
"""

import math
from typing import List, Optional


class BasicClass:
    """A basic class for testing Python parsing"""
    
    def __init__(self, value: int = 0, text: str = "default"):
        """Initialize the basic class with optional parameters"""
        self._private_field = value
        self.public_property = text
        self.items = []
    
    def add(self, a: int, b: int) -> int:
        """Add two numbers together"""
        return a + b
    
    def print_info(self):
        """Print information about this instance"""
        print(f"Field: {self._private_field}, Property: {self.public_property}")
    
    def _is_valid(self) -> bool:
        """Private method to check validity"""
        return self._private_field >= 0 and self.public_property is not None
    
    @staticmethod
    def static_method():
        """A static method"""
        print("Static method called")
    
    @classmethod
    def from_string(cls, text: str):
        """Create instance from string"""
        return cls(len(text), text)
    
    @property
    def field_value(self) -> int:
        """Property getter for private field"""
        return self._private_field
    
    @field_value.setter
    def field_value(self, value: int):
        """Property setter for private field"""
        if value >= 0:
            self._private_field = value


def standalone_function(x: float, y: float) -> float:
    """A standalone function outside any class"""
    return math.sqrt(x * x + y * y)


def function_with_nested_def():
    """Function containing nested function definition"""
    def inner_function(n: int) -> int:
        return n * 2
    
    result = inner_function(5)
    return result


class NestedClass:
    """A class with nested class definition"""
    
    class InnerClass:
        """Inner class definition"""
        def __init__(self, name: str):
            self.name = name
        
        def get_name(self) -> str:
            return self.name
    
    def __init__(self):
        self.inner = self.InnerClass("default")
    
    def create_inner(self, name: str):
        """Create a new inner class instance"""
        return self.InnerClass(name)