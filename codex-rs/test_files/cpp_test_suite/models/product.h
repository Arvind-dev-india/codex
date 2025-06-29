/**
 * Product model header for the C++ test suite
 */

#ifndef PRODUCT_H
#define PRODUCT_H

#include <string>
#include <vector>
#include <memory>
#include <chrono>
#include <map>

namespace Models {

/**
 * Product category class
 */
class Category {
private:
    int id;
    std::string name;
    std::string description;
    std::vector<class Product*> products; // Forward declaration

public:
    // Constructors
    Category();
    Category(int categoryId, const std::string& categoryName, const std::string& desc = "");
    Category(const Category& other);
    Category(Category&& other) noexcept;
    
    // Destructor
    ~Category() = default;
    
    // Assignment operators
    Category& operator=(const Category& other);
    Category& operator=(Category&& other) noexcept;
    
    // Methods
    void addProduct(Product* product);
    void removeProduct(Product* product);
    size_t getProductCount() const;
    std::vector<Product*> getProducts() const;
    
    // Accessors
    int getId() const { return id; }
    const std::string& getName() const { return name; }
    const std::string& getDescription() const { return description; }
    
    // Mutators
    void setId(int categoryId) { id = categoryId; }
    void setName(const std::string& categoryName) { name = categoryName; }
    void setDescription(const std::string& desc) { description = desc; }
    
    // Utility
    std::string toString() const;
    bool isValid() const;
    
    // Operators
    bool operator==(const Category& other) const;
    bool operator!=(const Category& other) const;
};

/**
 * Base Product class
 */
class Product {
protected:
    int id;
    std::string name;
    double price;
    std::string description;
    Category* category;
    int stockQuantity;
    std::chrono::system_clock::time_point createdAt;
    std::vector<std::string> tags;

public:
    // Constructors
    Product();
    Product(int productId, const std::string& productName, double productPrice, const std::string& desc = "");
    Product(const Product& other);
    Product(Product&& other) noexcept;
    
    // Virtual destructor for inheritance
    virtual ~Product() = default;
    
    // Assignment operators
    Product& operator=(const Product& other);
    Product& operator=(Product&& other) noexcept;
    
    // Virtual methods
    virtual void updatePrice(double newPrice);
    virtual bool isAvailable() const;
    virtual double calculateDiscountedPrice(double discountPercentage) const;
    virtual std::string getProductType() const { return "Product"; }
    virtual std::unique_ptr<Product> clone() const;
    
    // Stock management
    void addStock(int quantity);
    bool removeStock(int quantity);
    bool isInStock() const;
    
    // Tag management
    void addTag(const std::string& tag);
    void removeTag(const std::string& tag);
    bool hasTag(const std::string& tag) const;
    std::vector<std::string> getTags() const { return tags; }
    
    // Category management
    void setCategory(Category* cat);
    Category* getCategory() const { return category; }
    
    // Accessors
    int getId() const { return id; }
    const std::string& getName() const { return name; }
    double getPrice() const { return price; }
    const std::string& getDescription() const { return description; }
    int getStockQuantity() const { return stockQuantity; }
    const std::chrono::system_clock::time_point& getCreatedAt() const { return createdAt; }
    
    // Mutators
    void setId(int productId) { id = productId; }
    void setName(const std::string& productName) { name = productName; }
    void setDescription(const std::string& desc) { description = desc; }
    
    // Utility methods
    std::string toString() const;
    virtual bool isValid() const;
    
    // Operators
    bool operator==(const Product& other) const;
    bool operator!=(const Product& other) const;
    bool operator<(const Product& other) const; // For sorting
    friend std::ostream& operator<<(std::ostream& os, const Product& product);

protected:
    void logPriceChange(double oldPrice, double newPrice) const;
    void logStockChange(const std::string& message) const;
    void validatePrice(double price) const;
};

/**
 * Digital Product class - doesn't require physical stock
 */
class DigitalProduct : public Product {
private:
    std::string downloadUrl;
    int downloadCount;
    double fileSizeMB;
    std::string fileFormat;

public:
    // Constructors
    DigitalProduct();
    DigitalProduct(int productId, const std::string& productName, double productPrice, 
                   const std::string& url, const std::string& desc = "");
    DigitalProduct(const DigitalProduct& other);
    DigitalProduct(DigitalProduct&& other) noexcept;
    
    // Assignment operators
    DigitalProduct& operator=(const DigitalProduct& other);
    DigitalProduct& operator=(DigitalProduct&& other) noexcept;
    
    // Override virtual methods
    bool isAvailable() const override;
    std::string getProductType() const override { return "DigitalProduct"; }
    std::unique_ptr<Product> clone() const override;
    
    // Digital-specific methods
    void recordDownload();
    void updateDownloadUrl(const std::string& url);
    
    // Override stock methods (digital products don't need stock)
    void addStock(int quantity) { /* No-op for digital products */ }
    bool removeStock(int quantity) { return true; /* Always available */ }
    
    // Accessors
    const std::string& getDownloadUrl() const { return downloadUrl; }
    int getDownloadCount() const { return downloadCount; }
    double getFileSizeMB() const { return fileSizeMB; }
    const std::string& getFileFormat() const { return fileFormat; }
    
    // Mutators
    void setDownloadUrl(const std::string& url) { downloadUrl = url; }
    void setFileSizeMB(double size) { fileSizeMB = size; }
    void setFileFormat(const std::string& format) { fileFormat = format; }
    
    // Override utility methods
    std::string toString() const override;
    bool isValid() const override;

private:
    void logDownload() const;
    void validateUrl(const std::string& url) const;
};

/**
 * Physical Product class - requires inventory management
 */
class PhysicalProduct : public Product {
private:
    double weight;
    double length, width, height; // Dimensions
    std::string sku;
    int reorderLevel;
    int maxStockLevel;

public:
    // Constructors
    PhysicalProduct();
    PhysicalProduct(int productId, const std::string& productName, double productPrice,
                    double productWeight, const std::string& productSku, const std::string& desc = "");
    PhysicalProduct(const PhysicalProduct& other);
    PhysicalProduct(PhysicalProduct&& other) noexcept;
    
    // Assignment operators
    PhysicalProduct& operator=(const PhysicalProduct& other);
    PhysicalProduct& operator=(PhysicalProduct&& other) noexcept;
    
    // Override virtual methods
    std::string getProductType() const override { return "PhysicalProduct"; }
    std::unique_ptr<Product> clone() const override;
    
    // Physical-specific methods
    double calculateShippingCost(double distance) const;
    bool needsReorder() const;
    double getVolume() const;
    void setDimensions(double l, double w, double h);
    
    // Accessors
    double getWeight() const { return weight; }
    double getLength() const { return length; }
    double getWidth() const { return width; }
    double getHeight() const { return height; }
    const std::string& getSku() const { return sku; }
    int getReorderLevel() const { return reorderLevel; }
    int getMaxStockLevel() const { return maxStockLevel; }
    
    // Mutators
    void setWeight(double productWeight) { weight = productWeight; }
    void setSku(const std::string& productSku) { sku = productSku; }
    void setReorderLevel(int level) { reorderLevel = level; }
    void setMaxStockLevel(int level) { maxStockLevel = level; }
    
    // Override utility methods
    std::string toString() const override;
    bool isValid() const override;

private:
    void validateDimensions(double l, double w, double h) const;
    void validateWeight(double w) const;
};

/**
 * Product factory class
 */
class ProductFactory {
public:
    enum class ProductType {
        DIGITAL,
        PHYSICAL
    };
    
    static std::unique_ptr<Product> createProduct(ProductType type, int id, 
                                                 const std::string& name, double price);
    
    static std::unique_ptr<DigitalProduct> createDigitalProduct(int id, const std::string& name, 
                                                               double price, const std::string& url);
    
    static std::unique_ptr<PhysicalProduct> createPhysicalProduct(int id, const std::string& name, 
                                                                  double price, double weight, 
                                                                  const std::string& sku);
    
    // Template method for creating products with custom parameters
    template<typename ProductT, typename... Args>
    static std::unique_ptr<ProductT> createCustomProduct(Args&&... args);

private:
    static int generateId();
    static int nextId;
};

// Template implementation
template<typename ProductT, typename... Args>
std::unique_ptr<ProductT> ProductFactory::createCustomProduct(Args&&... args) {
    static_assert(std::is_base_of_v<Product, ProductT>, "ProductT must derive from Product");
    return std::make_unique<ProductT>(std::forward<Args>(args)...);
}

/**
 * Product catalog class for managing products
 */
class ProductCatalog {
private:
    std::map<int, std::unique_ptr<Product>> products;
    std::map<int, std::unique_ptr<Category>> categories;
    int nextProductId;
    int nextCategoryId;

public:
    ProductCatalog();
    ~ProductCatalog() = default;
    
    // Non-copyable but movable
    ProductCatalog(const ProductCatalog&) = delete;
    ProductCatalog& operator=(const ProductCatalog&) = delete;
    ProductCatalog(ProductCatalog&&) = default;
    ProductCatalog& operator=(ProductCatalog&&) = default;
    
    // Product management
    int addProduct(std::unique_ptr<Product> product);
    Product* getProduct(int productId);
    const Product* getProduct(int productId) const;
    bool removeProduct(int productId);
    
    // Category management
    int addCategory(std::unique_ptr<Category> category);
    Category* getCategory(int categoryId);
    const Category* getCategory(int categoryId) const;
    bool removeCategory(int categoryId);
    
    // Search and filter
    std::vector<Product*> searchProducts(const std::string& searchTerm);
    std::vector<Product*> getProductsByCategory(int categoryId);
    std::vector<Product*> getProductsByPriceRange(double minPrice, double maxPrice);
    std::vector<Product*> getProductsWithTag(const std::string& tag);
    
    // Statistics
    size_t getProductCount() const { return products.size(); }
    size_t getCategoryCount() const { return categories.size(); }
    double getAveragePrice() const;
    Product* getMostExpensiveProduct() const;
    Product* getCheapestProduct() const;
    
    // Template methods
    template<typename Predicate>
    std::vector<Product*> findProductsIf(Predicate pred);
    
    template<typename Comparator>
    std::vector<Product*> getSortedProducts(Comparator comp);

private:
    int generateProductId();
    int generateCategoryId();
};

// Template method implementations
template<typename Predicate>
std::vector<Product*> ProductCatalog::findProductsIf(Predicate pred) {
    std::vector<Product*> result;
    for (auto& pair : products) {
        if (pred(*pair.second)) {
            result.push_back(pair.second.get());
        }
    }
    return result;
}

template<typename Comparator>
std::vector<Product*> ProductCatalog::getSortedProducts(Comparator comp) {
    std::vector<Product*> result;
    result.reserve(products.size());
    
    for (auto& pair : products) {
        result.push_back(pair.second.get());
    }
    
    std::sort(result.begin(), result.end(), 
              [&comp](Product* a, Product* b) {
                  return comp(*a, *b);
              });
    
    return result;
}

} // namespace Models

#endif // PRODUCT_H