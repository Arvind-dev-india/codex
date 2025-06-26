"""
A test module for demonstrating line number detection
"""

import math
from typing import Optional

class Calculator:
    """A simple calculator class."""
    
    def __init__(self, initial_value: float = 0.0):
        """Initialize the calculator with an optional initial value."""
        self.value = initial_value
        self.history = []
    
    def add(self, x: float) -> float:
        """Add a number to the current value."""
        self.value += x
        self.history.append(f"Added {x}")
        return self.value

def simple_function():
    print("This is a simple function")
    return 42

class MathHelper:
    """Helper class for mathematical operations."""
    
    PI = 3.14159
    
    @staticmethod
    def calculate_circle_area(radius: float) -> float:
        """Calculate the area of a circle."""
        return MathHelper.PI * radius * radius
    
    @classmethod
    def from_diameter(cls, diameter: float):
        """Create a circle calculation from diameter."""
        return cls.calculate_circle_area(diameter / 2)