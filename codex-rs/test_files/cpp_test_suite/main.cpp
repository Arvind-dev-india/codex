/**
 * Main application file demonstrating cross-file references and complex C++ interactions
 */

#include <iostream>
#include <memory>
#include <vector>
#include <string>
#include <exception>

#include "models/user.h"
#include "models/order.h"
#include "models/product.h"
#include "services/user_service.h"
#include "data/repository.h"
#include "utils/helpers.h"
#include "basic_class.h"

using namespace Models;
using namespace Services;
using namespace Data;
using namespace Utils;

/**
 * Sample data creation function
 */
void createSampleData(UserService& userService, ProductCatalog& catalog) {
    Logger& logger = Logger::getInstance();
    ConfigManager config;
    
    logger.info("Creating sample data...");
    
    try {
        // Create sample users
        std::vector<std::pair<std::string, std::string>> userData = {
            {"John Doe", "john@example.com"},
            {"Jane Smith", "jane@example.com"},
            {"Bob Johnson", "bob@example.com"}
        };
        
        std::vector<std::unique_ptr<User>> users;
        for (const auto& [name, email] : userData) {
            if (StringUtils::validateEmail(email)) {
                auto user = userService.createUser(name, email);
                logger.info("Created user: " + user->getName());
                users.push_back(std::move(user));
            }
        }
        
        // Create sample categories
        auto electronicsCategory = std::make_unique<Category>(1, "Electronics", "Electronic devices and gadgets");
        auto softwareCategory = std::make_unique<Category>(2, "Software", "Digital software products");
        
        int electronicsId = catalog.addCategory(std::move(electronicsCategory));
        int softwareId = catalog.addCategory(std::move(softwareCategory));
        
        // Create sample products
        auto laptop = ProductFactory::createPhysicalProduct(1, "Gaming Laptop", 1299.99, 2.5, "LAP-001");
        laptop->setCategory(catalog.getCategory(electronicsId));
        laptop->addStock(10);
        laptop->addTag("gaming");
        laptop->addTag("portable");
        catalog.addProduct(std::move(laptop));
        
        auto mouse = ProductFactory::createPhysicalProduct(2, "Wireless Mouse", 49.99, 0.1, "MOU-001");
        mouse->setCategory(catalog.getCategory(electronicsId));
        mouse->addStock(25);
        mouse->addTag("wireless");
        catalog.addProduct(std::move(mouse));
        
        auto software = ProductFactory::createDigitalProduct(3, "Photo Editor Pro", 99.99, "https://download.example.com/photo-editor");
        software->setCategory(catalog.getCategory(softwareId));
        software->addTag("creative");
        software->addTag("professional");
        catalog.addProduct(std::move(software));
        
        logger.info("Sample data creation completed successfully");
        
    } catch (const std::exception& e) {
        logger.error("Error creating sample data: " + std::string(e.what()));
        throw;
    }
}

/**
 * Demonstrate functionality
 */
void demonstrateFunctionality() {
    Timer timer("Sample data creation");
    auto scopedTimer = timer.createScopedTimer();
    
    Logger& logger = Logger::getInstance();
    logger.setLevel(Logger::Level::INFO);
    logger.enableConsole(true);
    
    try {
        // Create repositories and services
        auto userRepo = RepositoryFactory::createInMemoryRepository<User>();
        auto userService = std::make_unique<UserService>(std::move(userRepo));
        
        ProductCatalog catalog;
        
        // Create sample data
        createSampleData(*userService, catalog);
        
        // Test basic class functionality
        auto basicObj = std::make_unique<DerivedClass>(42, "test", 3.14);
        basicObj->virtualMethod();
        basicObj->pureVirtualMethod();
        
        int result = basicObj->add(10, 20);
        logger.info("BasicClass add result: " + std::to_string(result));
        
        // Test standalone function
        standaloneFunction(3, 4);
        
        // Test template functionality
        auto templateObj = std::make_unique<TemplateClass<std::string>>("Hello");
        templateObj->addItem("World");
        templateObj->addItem("C++");
        
        logger.info("Template class size: " + std::to_string(templateObj->getSize()));
        
        // Test user search
        auto searchResults = userService->searchUsers("john");
        logger.info("Search results for 'john': " + std::to_string(searchResults.size()) + " users found");
        
        // Test product operations
        auto products = catalog.searchProducts("laptop");
        logger.info("Found " + std::to_string(products.size()) + " laptop products");
        
        // Test configuration
        ConfigManager config;
        config.set("app_name", "C++ Test Suite");
        config.set("version", "1.0.0");
        config.set<int>("max_users", 1000);
        
        logger.info("Application: " + config.get("app_name"));
        logger.info("Max users: " + std::to_string(config.get<int>("max_users")));
        
        // Test validation
        auto validationResult = DataValidator::validateEmail("test@example.com");
        if (validationResult.isValid) {
            logger.info("Email validation passed");
        } else {
            logger.error("Email validation failed: " + validationResult.getErrorsAsString());
        }
        
        // Test retry policy
        auto retryFunc = [&logger](int value) -> int {
            if (value < 5) {
                throw std::runtime_error("Value too small");
            }
            logger.info("Retry function succeeded with value: " + std::to_string(value));
            return value * 2;
        };
        
        auto retryPolicy = createRetryPolicy(retryFunc, 3);
        try {
            int retryResult = retryPolicy.execute(6);
            logger.info("Retry result: " + std::to_string(retryResult));
        } catch (const std::exception& e) {
            logger.error("Retry failed: " + std::string(e.what()));
        }
        
    } catch (const std::exception& e) {
        logger.error("Demonstration failed: " + std::string(e.what()));
        throw;
    }
}

/**
 * Application manager class
 */
class ApplicationManager {
private:
    ConfigManager config;
    Logger& logger;
    bool isRunning;

public:
    ApplicationManager() : logger(Logger::getInstance()), isRunning(false) {
        logger.setLevel(Logger::Level::INFO);
        config.set("app_name", "C++ Test Suite Application");
        config.set("version", "1.0.0");
    }
    
    ~ApplicationManager() {
        if (isRunning) {
            stop();
        }
    }
    
    void start() {
        logger.info("Starting application...");
        isRunning = true;
        
        try {
            demonstrateFunctionality();
        } catch (const std::exception& e) {
            logger.error("Application error: " + std::string(e.what()));
        }
        
        stop();
    }
    
    void stop() {
        logger.info("Stopping application...");
        isRunning = false;
    }
    
    struct Status {
        bool running;
        std::string appName;
        std::string version;
        size_t logCount;
    };
    
    Status getStatus() const {
        return Status{
            isRunning,
            config.get("app_name"),
            config.get("version"),
            logger.getLogCount()
        };
    }
};

/**
 * Template function for processing collections
 */
template<typename Container, typename Processor>
void processCollection(const Container& container, Processor processor) {
    Logger& logger = Logger::getInstance();
    logger.info("Processing collection of " + std::to_string(container.size()) + " items");
    
    for (const auto& item : container) {
        processor(item);
    }
}

/**
 * Function template specialization example
 */
template<typename T>
void printValue(const T& value) {
    std::cout << "Generic value: " << value << std::endl;
}

// Specialization for strings
template<>
void printValue<std::string>(const std::string& value) {
    std::cout << "String value: \"" << value << "\"" << std::endl;
}

/**
 * RAII resource manager example
 */
class ResourceManager {
private:
    std::string resourceName;
    bool acquired;

public:
    explicit ResourceManager(const std::string& name) : resourceName(name), acquired(false) {
        acquire();
    }
    
    ~ResourceManager() {
        if (acquired) {
            release();
        }
    }
    
    // Non-copyable but movable
    ResourceManager(const ResourceManager&) = delete;
    ResourceManager& operator=(const ResourceManager&) = delete;
    
    ResourceManager(ResourceManager&& other) noexcept 
        : resourceName(std::move(other.resourceName)), acquired(other.acquired) {
        other.acquired = false;
    }
    
    ResourceManager& operator=(ResourceManager&& other) noexcept {
        if (this != &other) {
            if (acquired) {
                release();
            }
            resourceName = std::move(other.resourceName);
            acquired = other.acquired;
            other.acquired = false;
        }
        return *this;
    }
    
    void acquire() {
        if (!acquired) {
            Logger::getInstance().info("Acquiring resource: " + resourceName);
            acquired = true;
        }
    }
    
    void release() {
        if (acquired) {
            Logger::getInstance().info("Releasing resource: " + resourceName);
            acquired = false;
        }
    }
    
    bool isAcquired() const { return acquired; }
    const std::string& getName() const { return resourceName; }
};

/**
 * Main function
 */
int main(int argc, char* argv[]) {
    try {
        // Initialize logger
        Logger& logger = Logger::getInstance();
        logger.info("C++ Test Suite Application Starting...");
        
        // Parse command line arguments
        std::vector<std::string> args(argv, argv + argc);
        if (args.size() > 1) {
            logger.info("Command line arguments provided: " + std::to_string(args.size() - 1));
            for (size_t i = 1; i < args.size(); ++i) {
                logger.info("  Arg " + std::to_string(i) + ": " + args[i]);
            }
        }
        
        // Create and run application
        ApplicationManager app;
        app.start();
        
        // Test template functions
        printValue(42);
        printValue(3.14);
        printValue(std::string("Hello, C++!"));
        
        // Test collection processing
        std::vector<int> numbers = {1, 2, 3, 4, 5};
        processCollection(numbers, [&logger](int n) {
            logger.info("Processing number: " + std::to_string(n));
        });
        
        // Test RAII resource management
        {
            ResourceManager resource("TestResource");
            logger.info("Resource acquired: " + std::string(resource.isAcquired() ? "true" : "false"));
        } // Resource automatically released here
        
        // Get final status
        auto status = app.getStatus();
        logger.info("Final application status:");
        logger.info("  Running: " + std::string(status.running ? "true" : "false"));
        logger.info("  App Name: " + status.appName);
        logger.info("  Version: " + status.version);
        logger.info("  Log Count: " + std::to_string(status.logCount));
        
        logger.info("C++ Test Suite Application completed successfully");
        return 0;
        
    } catch (const std::exception& e) {
        std::cerr << "Fatal error: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "Unknown fatal error occurred" << std::endl;
        return 2;
    }
}