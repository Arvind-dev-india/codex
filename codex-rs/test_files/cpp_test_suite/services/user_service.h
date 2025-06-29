/**
 * User service header for managing user operations
 */

#ifndef USER_SERVICE_H
#define USER_SERVICE_H

#include <memory>
#include <vector>
#include <string>
#include <functional>
#include "../models/user.h"
#include "../data/repository.h"

namespace Services {

/**
 * Interface for user service operations
 */
class IUserService {
public:
    virtual ~IUserService() = default;
    
    virtual std::unique_ptr<Models::User> createUser(const std::string& name, const std::string& email) = 0;
    virtual Models::User* getUser(int userId) = 0;
    virtual const Models::User* getUser(int userId) const = 0;
    virtual std::vector<Models::User*> getAllUsers() = 0;
    virtual bool updateUser(const Models::User& user) = 0;
    virtual bool deleteUser(int userId) = 0;
    virtual std::vector<Models::User*> searchUsers(const std::string& searchTerm) = 0;
};

/**
 * User service implementation
 */
class UserService : public IUserService {
private:
    std::unique_ptr<Data::Repository<Models::User>> userRepository;
    std::shared_ptr<class OrderService> orderService; // Forward declaration
    int nextUserId;

public:
    // Constructors
    explicit UserService(std::unique_ptr<Data::Repository<Models::User>> userRepo);
    UserService(std::unique_ptr<Data::Repository<Models::User>> userRepo, 
                std::shared_ptr<class OrderService> orderSvc);
    
    // Destructor
    ~UserService() override = default;
    
    // Non-copyable but movable
    UserService(const UserService&) = delete;
    UserService& operator=(const UserService&) = delete;
    UserService(UserService&&) = default;
    UserService& operator=(UserService&&) = default;
    
    // IUserService implementation
    std::unique_ptr<Models::User> createUser(const std::string& name, const std::string& email) override;
    Models::User* getUser(int userId) override;
    const Models::User* getUser(int userId) const override;
    std::vector<Models::User*> getAllUsers() override;
    bool updateUser(const Models::User& user) override;
    bool deleteUser(int userId) override;
    std::vector<Models::User*> searchUsers(const std::string& searchTerm) override;
    
    // Additional methods
    std::unique_ptr<Models::AdminUser> promoteToAdmin(int userId, int adminLevel = 1);
    bool demoteFromAdmin(int userId);
    std::vector<Models::AdminUser*> getAdminUsers();
    
    // User statistics
    size_t getUserCount() const;
    double getAverageOrderValue() const;
    Models::User* getMostActiveUser() const;
    std::vector<Models::User*> getUsersWithOrderCount(size_t minOrders) const;
    
    // Bulk operations
    void importUsers(const std::vector<std::pair<std::string, std::string>>& userData);
    bool exportUsers(const std::string& filename) const;
    
    // Template methods
    template<typename Predicate>
    std::vector<Models::User*> findUsersIf(Predicate pred);
    
    template<typename Comparator>
    void sortUsers(Comparator comp);
    
    // Event handling
    using UserEventHandler = std::function<void(const Models::User&, const std::string&)>;
    void setUserCreatedHandler(UserEventHandler handler) { userCreatedHandler = handler; }
    void setUserUpdatedHandler(UserEventHandler handler) { userUpdatedHandler = handler; }
    void setUserDeletedHandler(UserEventHandler handler) { userDeletedHandler = handler; }

private:
    // Event handlers
    UserEventHandler userCreatedHandler;
    UserEventHandler userUpdatedHandler;
    UserEventHandler userDeletedHandler;
    
    // Helper methods
    int generateUserId();
    bool isEmailExists(const std::string& email) const;
    void validateUserData(const std::string& name, const std::string& email) const;
    void loadUserOrders(Models::User& user) const;
    void validateUserUpdate(const Models::User& newUser, const Models::User& existingUser) const;
    bool hasActiveOrders(const Models::User& user) const;
    
    // Logging methods
    void logUserCreation(const Models::User& user) const;
    void logUserUpdate(const Models::User& user) const;
    void logUserDeletion(const Models::User& user) const;
    void logUserPromotion(const Models::AdminUser& adminUser) const;
    
    // Event notification methods
    void notifyUserCreated(const Models::User& user) const;
    void notifyUserUpdated(const Models::User& user) const;
    void notifyUserDeleted(const Models::User& user) const;
};

// Template method implementations
template<typename Predicate>
std::vector<Models::User*> UserService::findUsersIf(Predicate pred) {
    auto allUsers = getAllUsers();
    std::vector<Models::User*> result;
    
    std::copy_if(allUsers.begin(), allUsers.end(), std::back_inserter(result), pred);
    return result;
}

template<typename Comparator>
void UserService::sortUsers(Comparator comp) {
    // This would require access to the underlying container
    // Implementation depends on repository design
    auto allUsers = getAllUsers();
    std::sort(allUsers.begin(), allUsers.end(), 
              [&comp](Models::User* a, Models::User* b) {
                  return comp(*a, *b);
              });
    
    // Note: This sorts the pointers, not the actual storage
    // A real implementation would need to update the repository
}

/**
 * User service factory
 */
class UserServiceFactory {
public:
    static std::unique_ptr<UserService> createUserService();
    static std::unique_ptr<UserService> createUserServiceWithOrderService(
        std::shared_ptr<class OrderService> orderService);
    
    // Template factory method
    template<typename RepositoryType>
    static std::unique_ptr<UserService> createUserServiceWithRepository();

private:
    static std::unique_ptr<Data::Repository<Models::User>> createDefaultRepository();
};

// Template implementation
template<typename RepositoryType>
std::unique_ptr<UserService> UserServiceFactory::createUserServiceWithRepository() {
    static_assert(std::is_base_of_v<Data::Repository<Models::User>, RepositoryType>, 
                  "RepositoryType must derive from Repository<User>");
    
    auto repository = std::make_unique<RepositoryType>();
    return std::make_unique<UserService>(std::move(repository));
}

/**
 * User service configuration
 */
struct UserServiceConfig {
    bool enableEmailValidation = true;
    bool enableEventLogging = true;
    bool enableOrderLoading = true;
    size_t maxUsersPerSearch = 100;
    std::string logLevel = "INFO";
    
    // Validation settings
    size_t maxNameLength = 100;
    size_t maxEmailLength = 100;
    size_t minPasswordLength = 8;
    
    // Default values
    static UserServiceConfig getDefault();
    static UserServiceConfig getTestConfig();
    
    // Validation
    bool isValid() const;
    std::string toString() const;
};

/**
 * User service builder for complex configuration
 */
class UserServiceBuilder {
private:
    std::unique_ptr<Data::Repository<Models::User>> repository;
    std::shared_ptr<class OrderService> orderService;
    UserServiceConfig config;
    std::vector<UserService::UserEventHandler> eventHandlers;

public:
    UserServiceBuilder();
    
    // Fluent interface
    UserServiceBuilder& withRepository(std::unique_ptr<Data::Repository<Models::User>> repo);
    UserServiceBuilder& withOrderService(std::shared_ptr<class OrderService> orderSvc);
    UserServiceBuilder& withConfig(const UserServiceConfig& cfg);
    UserServiceBuilder& withEventHandler(UserService::UserEventHandler handler);
    UserServiceBuilder& enableEmailValidation(bool enable = true);
    UserServiceBuilder& enableEventLogging(bool enable = true);
    UserServiceBuilder& withMaxUsersPerSearch(size_t maxUsers);
    
    // Build method
    std::unique_ptr<UserService> build();
    
    // Reset for reuse
    void reset();

private:
    void validateConfiguration() const;
    void applyConfiguration(UserService& service) const;
};

} // namespace Services

#endif // USER_SERVICE_H