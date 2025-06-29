/**
 * Header file for basic C++ classes and functions
 */

#ifndef BASIC_CLASS_H
#define BASIC_CLASS_H

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
    // Constructors
    BasicClass();
    BasicClass(int value, const std::string& text);
    BasicClass(const BasicClass& other);
    
    // Destructor
    virtual ~BasicClass() = default;
    
    // Methods
    int add(int a, int b) const;
    void printInfo() const;
    virtual void virtualMethod();
    virtual void pureVirtualMethod() = 0;
    static void staticMethod();
    
    // Operators
    BasicClass& operator=(const BasicClass& other);
    
    // Accessors
    int getPrivateField() const;
    const std::string& getPublicProperty() const;
    void setPrivateField(int value);
    void setPublicProperty(const std::string& value);

private:
    bool isValid() const;
};

/**
 * Template class declaration
 */
template<typename T>
class TemplateClass {
private:
    T data;
    std::vector<T> collection;

public:
    explicit TemplateClass(const T& value);
    void addItem(const T& item);
    T getData() const;
    size_t getSize() const;
    
    template<typename U>
    void processData(const U& processor);
};

/**
 * Derived class declaration
 */
class DerivedClass : public BasicClass {
private:
    double additionalField;

public:
    DerivedClass(int value, const std::string& text, double additional);
    
    void virtualMethod() override;
    void pureVirtualMethod() override;
    double getAdditionalField() const;
    
    void processData(int value);
    void processData(const std::string& value);
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
        explicit InnerClass(const std::string& data);
        void printInnerData() const;
        const std::string& getData() const;
    };
    
private:
    InnerClass inner;
    
public:
    explicit OuterClass(const std::string& data);
    void processInner();
    InnerClass& getInner();
    const InnerClass& getInner() const;
};

// Function declarations
void standaloneFunction(int x, int y);
void functionWithDefaults(int a, int b = 10, const std::string& c = "default");

template<typename T>
T maxValue(const T& a, const T& b);

void processWithCallback(int value, void (*callback)(int));
auto createLambda();

// Namespace
namespace TestNamespace {
    class NamespaceClass {
    public:
        void namespaceMethod();
    };
    
    void namespaceFunction();
}

// Enums
enum class Color {
    RED,
    GREEN,
    BLUE
};

enum Status {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED
};

// Struct
struct Point {
    double x, y;
    
    Point(double x = 0.0, double y = 0.0);
    double distance() const;
    Point operator+(const Point& other) const;
};

// Union
union Data {
    int intValue;
    float floatValue;
    char charValue;
    
    Data(int value);
};

// Function prototypes
void declaredFunction();
int calculateSum(const std::vector<int>& numbers);
std::unique_ptr<BasicClass> createBasicClass(int value, const std::string& text);

#endif // BASIC_CLASS_H