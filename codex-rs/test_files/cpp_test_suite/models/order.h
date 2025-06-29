/**
 * Order model header for the C++ test suite
 */

#ifndef ORDER_H
#define ORDER_H

#include <string>
#include <vector>
#include <memory>
#include <chrono>
#include "product.h"

namespace Models {

/**
 * Enumeration for order status
 */
enum class OrderStatus {
    PENDING,
    PROCESSING,
    SHIPPED,
    DELIVERED,
    CANCELLED
};

// Helper function to convert enum to string
std::string orderStatusToString(OrderStatus status);
OrderStatus stringToOrderStatus(const std::string& status);

/**
 * Order item class representing a product in an order
 */
class OrderItem {
private:
    std::shared_ptr<Product> product;
    int quantity;
    double unitPrice; // Price at time of order

public:
    // Constructors
    OrderItem(std::shared_ptr<Product> prod, int qty);
    OrderItem(std::shared_ptr<Product> prod, int qty, double price);
    OrderItem(const OrderItem& other) = default;
    OrderItem(OrderItem&& other) = default;
    
    // Assignment operators
    OrderItem& operator=(const OrderItem& other) = default;
    OrderItem& operator=(OrderItem&& other) = default;
    
    // Destructor
    ~OrderItem() = default;
    
    // Methods
    double getTotalPrice() const;
    void updateQuantity(int newQuantity);
    
    // Accessors
    std::shared_ptr<Product> getProduct() const { return product; }
    int getQuantity() const { return quantity; }
    double getUnitPrice() const { return unitPrice; }
    
    // Utility
    std::string toString() const;
    bool isValid() const;
    
    // Operators
    bool operator==(const OrderItem& other) const;
    bool operator!=(const OrderItem& other) const;
};

/**
 * Order class representing a customer order
 */
class Order {
private:
    int id;
    int userId;
    OrderStatus status;
    std::chrono::system_clock::time_point createdAt;
    std::chrono::system_clock::time_point updatedAt;
    std::vector<OrderItem> items;
    double discountPercentage;
    double taxRate;
    std::string notes;

public:
    // Constructors
    Order();
    Order(int orderId, int customerId);
    Order(const Order& other);
    Order(Order&& other) noexcept;
    
    // Destructor
    virtual ~Order() = default;
    
    // Assignment operators
    Order& operator=(const Order& other);
    Order& operator=(Order&& other) noexcept;
    
    // Item management
    void addItem(std::shared_ptr<Product> product, int quantity = 1);
    void addItem(const OrderItem& item);
    bool removeItem(int productId);
    void updateItemQuantity(int productId, int newQuantity);
    OrderItem* findItem(int productId);
    const OrderItem* findItem(int productId) const;
    
    // Calculations
    double calculateSubtotal() const;
    double calculateDiscount() const;
    double calculateTax() const;
    double calculateTotal() const;
    
    // Status management
    void updateStatus(OrderStatus newStatus);
    bool canTransitionTo(OrderStatus newStatus) const;
    
    // Accessors
    int getId() const { return id; }
    int getUserId() const { return userId; }
    OrderStatus getStatus() const { return status; }
    const std::chrono::system_clock::time_point& getCreatedAt() const { return createdAt; }
    const std::chrono::system_clock::time_point& getUpdatedAt() const { return updatedAt; }
    const std::vector<OrderItem>& getItems() const { return items; }
    double getDiscountPercentage() const { return discountPercentage; }
    double getTaxRate() const { return taxRate; }
    const std::string& getNotes() const { return notes; }
    
    // Mutators
    void setId(int orderId) { id = orderId; }
    void setUserId(int customerId) { userId = customerId; }
    void setDiscountPercentage(double discount);
    void setTaxRate(double rate);
    void setNotes(const std::string& orderNotes) { notes = orderNotes; }
    
    // Utility methods
    size_t getItemCount() const { return items.size(); }
    bool isEmpty() const { return items.empty(); }
    std::string toString() const;
    bool isValid() const;
    
    // Operators
    bool operator==(const Order& other) const;
    bool operator!=(const Order& other) const;
    friend std::ostream& operator<<(std::ostream& os, const Order& order);

private:
    void updateTimestamp();
    void logStatusChange(OrderStatus oldStatus, OrderStatus newStatus);
    void validateDiscount(double discount) const;
    void validateTaxRate(double rate) const;
};

/**
 * Order builder class for creating orders with fluent interface
 */
class OrderBuilder {
private:
    std::unique_ptr<Order> order;

public:
    OrderBuilder();
    explicit OrderBuilder(int orderId, int userId);
    
    // Fluent interface methods
    OrderBuilder& withId(int orderId);
    OrderBuilder& withUserId(int userId);
    OrderBuilder& withStatus(OrderStatus status);
    OrderBuilder& withDiscount(double percentage);
    OrderBuilder& withTaxRate(double rate);
    OrderBuilder& withNotes(const std::string& notes);
    OrderBuilder& addProduct(std::shared_ptr<Product> product, int quantity = 1);
    OrderBuilder& addItem(const OrderItem& item);
    
    // Build method
    std::unique_ptr<Order> build();
    
    // Reset for reuse
    void reset();

private:
    void ensureOrderExists();
};

/**
 * Order statistics class
 */
class OrderStatistics {
public:
    struct Stats {
        size_t totalOrders;
        double totalValue;
        double averageOrderValue;
        size_t totalItems;
        double averageItemsPerOrder;
        std::map<OrderStatus, size_t> statusCounts;
    };
    
    static Stats calculateStats(const std::vector<Order>& orders);
    static Stats calculateStats(const std::vector<std::shared_ptr<Order>>& orders);
    
    template<typename Container>
    static Stats calculateStatsGeneric(const Container& orders);
    
private:
    static void updateStats(Stats& stats, const Order& order);
};

// Template implementation
template<typename Container>
OrderStatistics::Stats OrderStatistics::calculateStatsGeneric(const Container& orders) {
    Stats stats = {};
    
    for (const auto& order : orders) {
        if constexpr (std::is_pointer_v<typename Container::value_type>) {
            updateStats(stats, *order);
        } else if constexpr (std::is_same_v<typename Container::value_type, std::shared_ptr<Order>>) {
            updateStats(stats, *order);
        } else {
            updateStats(stats, order);
        }
    }
    
    if (stats.totalOrders > 0) {
        stats.averageOrderValue = stats.totalValue / stats.totalOrders;
        stats.averageItemsPerOrder = static_cast<double>(stats.totalItems) / stats.totalOrders;
    }
    
    return stats;
}

} // namespace Models

#endif // ORDER_H