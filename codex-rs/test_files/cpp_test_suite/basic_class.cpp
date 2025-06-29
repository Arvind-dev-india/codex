/**
 * Basic C++ classes and functions for testing code analysis
 */

#include <iostream>
#include <string>
#include <vector>
#include <memory>

// Forward declarations
class NestedClass;

/**
 * A basic class for testing C++ parsing
 */
class BasicClass {
private:
    int privateField;
    std::string publicProperty;
    std::vector<std::string> items;

public:
    // Default constructor
    BasicClass() : privateField(0), publicProperty("default") {}
    
    // Parameterized constructor
    BasicClass(int value, const std::string& text) 
        : privateField(value), publicProperty(text) {}
    
    // Copy constructor
    BasicClass(const BasicClass& other) 
        : privateField(other.privateField), publicProperty(other.publicProperty) {}
    
    // Destructor
    virtual ~BasicClass() = default;
    
    // Basic method
    int add(int a, int b) const {
        return a + b;
    }
    
    // Method with reference parameters
    void printInfo() const {
        std::cout << "Field: " << privateField 
                  << ", Property: " << publicProperty << std::endl;
    }
    
    // Virtual method
    virtual void virtualMethod() {
        std::cout << "BasicClass virtual method" << std::endl;
    }
    
    // Pure virtual method (makes this an abstract class)
    virtual void pureVirtualMethod() = 0;
    
    // Static method
    static void staticMethod() {
        std::cout << "Static method called" << std::endl;
    }
    
    // Operator overloading
    BasicClass& operator=(const BasicClass& other) {
        if (this != &other) {
            privateField = other.privateField;
            publicProperty = other.publicProperty;
        }
        return *this;
    }
    
    // Getter methods
    int getPrivateField() const { return privateField; }
    const std::string& getPublicProperty() const { return publicProperty; }
    
    // Setter methods
    void setPrivateField(int value) { privateField = value; }
    void setPublicProperty(const std::string& value) { publicProperty = value; }

private:
    // Private helper method
    bool isValid() const {
        return privateField >= 0 && !publicProperty.empty();
    }
};

/**
 * Template class for testing generic programming
 */
template<typename T>
class TemplateClass {
private:
    T data;
    std::vector<T> collection;

public:
    explicit TemplateClass(const T& value) : data(value) {}
    
    void addItem(const T& item) {
        collection.push_back(item);
    }
    
    T getData() const { return data; }
    
    size_t getSize() const { return collection.size(); }
    
    // Template method
    template<typename U>
    void processData(const U& processor) {
        processor(data);
    }
};

/**
 * Derived class for testing inheritance
 */
class DerivedClass : public BasicClass {
private:
    double additionalField;

public:
    DerivedClass(int value, const std::string& text, double additional)
        : BasicClass(value, text), additionalField(additional) {}
    
    // Override virtual method
    void virtualMethod() override {
        std::cout << "DerivedClass virtual method" << std::endl;
        BasicClass::virtualMethod(); // Call base class method
    }
    
    // Implement pure virtual method
    void pureVirtualMethod() override {
        std::cout << "DerivedClass implementation of pure virtual method" << std::endl;
    }
    
    // Additional method
    double getAdditionalField() const { return additionalField; }
    
    // Method overloading
    void processData(int value) {
        std::cout << "Processing int: " << value << std::endl;
    }
    
    void processData(const std::string& value) {
        std::cout << "Processing string: " << value << std::endl;
    }
};

/**
 * Class with nested class
 */
class OuterClass {
public:
    class InnerClass {
    private:
        std::string innerData;
        
    public:
        explicit InnerClass(const std::string& data) : innerData(data) {}
        
        void printInnerData() const {
            std::cout << "Inner data: " << innerData << std::endl;
        }
        
        const std::string& getData() const { return innerData; }
    };
    
private:
    InnerClass inner;
    
public:
    explicit OuterClass(const std::string& data) : inner(data) {}
    
    void processInner() {
        inner.printInnerData();
    }
    
    InnerClass& getInner() { return inner; }
    const InnerClass& getInner() const { return inner; }
};

// Standalone functions
void standaloneFunction(int x, int y) {
    std::cout << "Standalone function: " << x << ", " << y << std::endl;
}

// Function with default parameters
void functionWithDefaults(int a, int b = 10, const std::string& c = "default") {
    std::cout << "Function with defaults: " << a << ", " << b << ", " << c << std::endl;
}

// Function template
template<typename T>
T maxValue(const T& a, const T& b) {
    return (a > b) ? a : b;
}

// Function with function pointer parameter
void processWithCallback(int value, void (*callback)(int)) {
    callback(value * 2);
}

// Lambda function demonstration
auto createLambda() {
    return [](int x, int y) -> int {
        return x + y;
    };
}

// Namespace demonstration
namespace TestNamespace {
    class NamespaceClass {
    public:
        void namespaceMethod() {
            std::cout << "Method in namespace" << std::endl;
        }
    };
    
    void namespaceFunction() {
        std::cout << "Function in namespace" << std::endl;
    }
}

// Enum class (C++11)
enum class Color {
    RED,
    GREEN,
    BLUE
};

// Traditional enum
enum Status {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED
};

// Struct (similar to class but public by default)
struct Point {
    double x, y;
    
    Point(double x = 0.0, double y = 0.0) : x(x), y(y) {}
    
    double distance() const {
        return std::sqrt(x * x + y * y);
    }
    
    Point operator+(const Point& other) const {
        return Point(x + other.x, y + other.y);
    }
};

// Union
union Data {
    int intValue;
    float floatValue;
    char charValue;
    
    Data(int value) : intValue(value) {}
};

// Function declarations (prototypes)
void declaredFunction();
int calculateSum(const std::vector<int>& numbers);
std::unique_ptr<BasicClass> createBasicClass(int value, const std::string& text);