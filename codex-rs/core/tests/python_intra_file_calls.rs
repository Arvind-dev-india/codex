use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType, ReferenceType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_python_intra_file_method_calls() {
    // Create a temporary directory and file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("IntraFileCalls.py");
    
    let content = r#"
import math
from typing import List, Optional

class Calculator:
    """A calculator class with intra-file method calls"""
    
    def __init__(self, initial_value: float = 0.0):
        self.value = initial_value
        self.history: List[str] = []
        self._reset_if_needed()  # Call to private method
    
    def add(self, x: float) -> float:
        """Add a number to the current value"""
        self.value += x
        self._log_operation(f"Added {x}")  # Call to private method
        return self.get_value()  # Call to public method
    
    def subtract(self, x: float) -> float:
        """Subtract a number from the current value"""
        self.value -= x
        self._log_operation(f"Subtracted {x}")  # Call to private method
        return self.get_value()  # Call to public method
    
    def multiply(self, x: float) -> float:
        """Multiply the current value"""
        self.value *= x
        self._log_operation(f"Multiplied by {x}")  # Call to private method
        if self._is_large_number():  # Call to private method
            self._handle_large_number()  # Call to private method
        return self.get_value()  # Call to public method
    
    def divide(self, x: float) -> float:
        """Divide the current value"""
        if x == 0:
            raise ValueError("Cannot divide by zero")
        self.value /= x
        self._log_operation(f"Divided by {x}")  # Call to private method
        return self.get_value()  # Call to public method
    
    def power(self, exponent: float) -> float:
        """Raise the current value to a power"""
        self.value = math.pow(self.value, exponent)
        self._log_operation(f"Raised to power {exponent}")  # Call to private method
        return self.get_value()  # Call to public method
    
    def sqrt(self) -> float:
        """Calculate square root of current value"""
        if self.value < 0:
            raise ValueError("Cannot calculate square root of negative number")
        self.value = math.sqrt(self.value)
        self._log_operation("Calculated square root")  # Call to private method
        return self.get_value()  # Call to public method
    
    def get_value(self) -> float:
        """Get the current value"""
        return self.value
    
    def get_history(self) -> List[str]:
        """Get the operation history"""
        return self.history.copy()
    
    def clear(self):
        """Clear the calculator"""
        self.value = 0.0
        self.history.clear()
        self._log_operation("Calculator cleared")  # Call to private method
    
    def complex_calculation(self, a: float, b: float, c: float) -> float:
        """Perform a complex calculation using multiple methods"""
        # Chain multiple method calls
        self.add(a)           # Call to public method
        self.multiply(b)      # Call to public method
        self.subtract(c)      # Call to public method
        
        # Use helper methods
        if self._is_negative():  # Call to private method
            self._make_positive()  # Call to private method
        
        result = self.get_value()  # Call to public method
        self._log_operation(f"Complex calculation result: {result}")  # Call to private method
        return result
    
    def _log_operation(self, operation: str):
        """Private method to log operations"""
        self.history.append(operation)
        if len(self.history) > 100:
            self._trim_history()  # Call to another private method
    
    def _trim_history(self):
        """Private method to trim history"""
        self.history = self.history[-50:]  # Keep last 50 operations
    
    def _is_large_number(self) -> bool:
        """Private method to check if number is large"""
        return abs(self.value) > 1000000
    
    def _handle_large_number(self):
        """Private method to handle large numbers"""
        self._log_operation("Warning: Large number detected")  # Call to private method
    
    def _is_negative(self) -> bool:
        """Private method to check if value is negative"""
        return self.value < 0
    
    def _make_positive(self):
        """Private method to make value positive"""
        self.value = abs(self.value)
        self._log_operation("Made value positive")  # Call to private method
    
    def _reset_if_needed(self):
        """Private method called from constructor"""
        if self.value is None:
            self.value = 0.0
            self._log_operation("Reset value to 0")  # Call to private method

class ScientificCalculator(Calculator):
    """Extended calculator with scientific functions"""
    
    def __init__(self, initial_value: float = 0.0):
        super().__init__(initial_value)  # Call to parent constructor
        self.angle_mode = "radians"
    
    def sin(self) -> float:
        """Calculate sine of current value"""
        if self.angle_mode == "degrees":
            radians = math.radians(self.value)
        else:
            radians = self.value
        
        self.value = math.sin(radians)
        self._log_operation(f"Calculated sin in {self.angle_mode} mode")  # Call to inherited method
        return self.get_value()  # Call to inherited method
    
    def cos(self) -> float:
        """Calculate cosine of current value"""
        if self.angle_mode == "degrees":
            radians = math.radians(self.value)
        else:
            radians = self.value
        
        self.value = math.cos(radians)
        self._log_operation(f"Calculated cos in {self.angle_mode} mode")  # Call to inherited method
        return self.get_value()  # Call to inherited method
    
    def set_angle_mode(self, mode: str):
        """Set angle mode (degrees or radians)"""
        if mode not in ["degrees", "radians"]:
            raise ValueError("Mode must be 'degrees' or 'radians'")
        self.angle_mode = mode
        self._log_operation(f"Set angle mode to {mode}")  # Call to inherited method
    
    def factorial(self) -> float:
        """Calculate factorial of current value"""
        if self.value < 0 or self.value != int(self.value):
            raise ValueError("Factorial requires non-negative integer")
        
        result = math.factorial(int(self.value))
        self.value = float(result)
        self._log_operation(f"Calculated factorial")  # Call to inherited method
        return self.get_value()  # Call to inherited method

def create_calculator(calc_type: str = "basic") -> Calculator:
    """Factory function that creates calculators"""
    if calc_type == "scientific":
        calc = ScientificCalculator()  # Call to constructor
    else:
        calc = Calculator()  # Call to constructor
    
    calc.clear()  # Call to method
    return calc

def test_calculator_operations():
    """Test function that demonstrates method calls"""
    calc = create_calculator("basic")  # Call to factory function
    
    # Chain of method calls
    calc.add(10)        # Call to method
    calc.multiply(2)    # Call to method
    calc.subtract(5)    # Call to method
    
    result = calc.get_value()  # Call to method
    history = calc.get_history()  # Call to method
    
    print(f"Result: {result}")
    print(f"History: {history}")
    
    # Test scientific calculator
    sci_calc = create_calculator("scientific")  # Call to factory function
    sci_calc.add(math.pi)  # Call to method
    sci_calc.sin()         # Call to method
    
    return calc, sci_calc

if __name__ == "__main__":
    test_calculator_operations()  # Call to function
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    let references = extractor.get_references();
    
    println!("Found {} symbols and {} references", symbols.len(), references.len());
    
    // Print all symbols for debugging
    for (fqn, symbol) in symbols {
        println!("Symbol: {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Print all references for debugging
    for (i, reference) in references.iter().enumerate() {
        println!("Reference {}: {} ({:?}) at line {}", 
                 i, reference.symbol_name, reference.reference_type, reference.reference_line);
    }
    
    // Test that we found the expected classes
    let calculator_class = symbols.values()
        .find(|s| s.name == "Calculator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Calculator class should be found");
    
    let scientific_calc_class = symbols.values()
        .find(|s| s.name == "ScientificCalculator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("ScientificCalculator class should be found");
    
    assert!(calculator_class.start_line > 0);
    assert!(scientific_calc_class.start_line > calculator_class.end_line);
    
    // Test that we found the expected methods (in Python, methods are detected as functions)
    let expected_methods = [
        "__init__", "add", "subtract", "multiply", "divide", "power", "sqrt",
        "get_value", "get_history", "clear", "complex_calculation",
        "_log_operation", "_trim_history", "_is_large_number", "_handle_large_number",
        "_is_negative", "_make_positive", "_reset_if_needed",
        "sin", "cos", "set_angle_mode", "factorial"
    ];
    
    for method_name in &expected_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
    
    // Test that we found the expected functions
    let expected_functions = ["create_calculator", "test_calculator_operations"];
    for func_name in &expected_functions {
        let function = symbols.values()
            .find(|s| s.name == *func_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} function should be found", func_name));
        
        assert!(function.start_line > 0);
        assert!(function.end_line > function.start_line);
    }
    
    // Test that we found method call references
    let method_call_refs: Vec<_> = references.iter()
        .filter(|r| matches!(r.reference_type, ReferenceType::Call))
        .collect();
    
    println!("Found {} method call references", method_call_refs.len());
    
    // Should find multiple method calls
    assert!(method_call_refs.len() >= 20, "Should find at least 20 method calls, found {}", method_call_refs.len());
    
    // Test specific method call references
    let add_calls: Vec<_> = method_call_refs.iter()
        .filter(|r| r.symbol_name == "add")
        .collect();
    
    let get_value_calls: Vec<_> = method_call_refs.iter()
        .filter(|r| r.symbol_name == "get_value")
        .collect();
    
    let log_operation_calls: Vec<_> = method_call_refs.iter()
        .filter(|r| r.symbol_name == "_log_operation")
        .collect();
    
    println!("Found {} calls to 'add'", add_calls.len());
    println!("Found {} calls to 'get_value'", get_value_calls.len());
    println!("Found {} calls to '_log_operation'", log_operation_calls.len());
    
    assert!(add_calls.len() >= 2, "Should find at least 2 calls to 'add' method");
    assert!(get_value_calls.len() >= 5, "Should find at least 5 calls to 'get_value' method");
    assert!(log_operation_calls.len() >= 10, "Should find at least 10 calls to '_log_operation' method");
}

#[test]
fn test_python_nested_function_calls() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("NestedCalls.py");
    
    let content = r#"
def outer_function(x: int) -> int:
    """Function with nested function calls"""
    
    def inner_function(y: int) -> int:
        """Nested function"""
        return y * 2
    
    def another_inner(z: int) -> int:
        """Another nested function"""
        return inner_function(z) + 1  # Call to sibling nested function
    
    result = inner_function(x)  # Call to nested function
    result = another_inner(result)  # Call to another nested function
    return helper_function(result)  # Call to module-level function

def helper_function(value: int) -> int:
    """Helper function at module level"""
    return value + 10

class NestedCallsClass:
    """Class with methods that call each other"""
    
    def method_a(self, x: int) -> int:
        """Method that calls other methods"""
        result = self.method_b(x)  # Call to instance method
        return self.method_c(result)  # Call to another instance method
    
    def method_b(self, x: int) -> int:
        """Method that calls helper"""
        return self._helper_method(x * 2)  # Call to private method
    
    def method_c(self, x: int) -> int:
        """Method that calls static method"""
        return self.static_method(x) + self._helper_method(1)  # Calls to static and private methods
    
    def _helper_method(self, x: int) -> int:
        """Private helper method"""
        return x + 5
    
    @staticmethod
    def static_method(x: int) -> int:
        """Static method"""
        return helper_function(x)  # Call to module-level function

def main():
    """Main function demonstrating all calls"""
    # Function calls
    result1 = outer_function(10)  # Call to function
    result2 = helper_function(20)  # Call to function
    
    # Class instantiation and method calls
    obj = NestedCallsClass()  # Constructor call
    result3 = obj.method_a(30)  # Method call
    result4 = obj.method_b(40)  # Method call
    result5 = obj.method_c(50)  # Method call
    
    # Static method call
    result6 = NestedCallsClass.static_method(60)  # Static method call
    
    return [result1, result2, result3, result4, result5, result6]

if __name__ == "__main__":
    main()  # Call to main function
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    let references = extractor.get_references();
    
    println!("Found {} symbols and {} references in nested calls test", symbols.len(), references.len());
    
    // Test nested function detection
    let outer_func = symbols.values()
        .find(|s| s.name == "outer_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("outer_function should be found");
    
    let inner_func = symbols.values()
        .find(|s| s.name == "inner_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("inner_function should be found");
    
    let another_inner = symbols.values()
        .find(|s| s.name == "another_inner" && matches!(s.symbol_type, SymbolType::Function))
        .expect("another_inner should be found");
    
    // Nested functions should be within the outer function's line range
    assert!(inner_func.start_line > outer_func.start_line);
    assert!(inner_func.end_line < outer_func.end_line);
    assert!(another_inner.start_line > outer_func.start_line);
    assert!(another_inner.end_line < outer_func.end_line);
    
    // Test method call references
    let method_calls: Vec<_> = references.iter()
        .filter(|r| matches!(r.reference_type, ReferenceType::Call))
        .collect();
    
    println!("Found {} method/function calls", method_calls.len());
    
    // Should find calls to nested functions, methods, and static methods
    assert!(method_calls.len() >= 10, "Should find at least 10 function/method calls");
    
    // Test specific call patterns
    let inner_function_calls: Vec<_> = method_calls.iter()
        .filter(|r| r.symbol_name == "inner_function")
        .collect();
    
    let helper_method_calls: Vec<_> = method_calls.iter()
        .filter(|r| r.symbol_name == "_helper_method")
        .collect();
    
    println!("Found {} calls to inner_function", inner_function_calls.len());
    println!("Found {} calls to _helper_method", helper_method_calls.len());
    
    assert!(inner_function_calls.len() >= 2, "Should find at least 2 calls to inner_function");
    assert!(helper_method_calls.len() >= 2, "Should find at least 2 calls to _helper_method");
}