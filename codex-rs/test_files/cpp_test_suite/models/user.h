/**
 * User model header for the C++ test suite
 */

#ifndef USER_H
#define USER_H

#include <string>
#include <vector>
#include <memory>
#include <chrono>
#include "order.h"

namespace Models {

/**
 * User class representing a user in the system
 */
class User {
private:
    int id;
    std::string name;
    std::string email;
    std::chrono::system_clock::time_point createdAt;
    std::vector<std::shared_ptr<Order>> orders;

public:
    // Constructors
    User();
    User(int userId, const std::string& userName, const std::string& userEmail);
    User(const User& other);
    User(User&& other) noexcept;
    
    // Destructor
    virtual ~User() = default;
    
    // Assignment operators
    User& operator=(const User& other);
    User& operator=(User&& other) noexcept;
    
    // Methods
    void addOrder(std::shared_ptr<Order> order);
    std::shared_ptr<Order> getOrder(int orderId) const;
    double getTotalOrderValue() const;
    size_t getOrderCount() const;
    
    // Accessors
    int getId() const { return id; }
    const std::string& getName() const { return name; }
    const std::string& getEmail() const { return email; }
    const std::chrono::system_clock::time_point& getCreatedAt() const { return createdAt; }
    const std::vector<std::shared_ptr<Order>>& getOrders() const { return orders; }
    
    // Mutators
    void setId(int userId) { id = userId; }
    void setName(const std::string& userName) { name = userName; }
    void setEmail(const std::string& userEmail) { email = userEmail; }
    
    // Utility methods
    std::string toString() const;
    bool isValid() const;
    
    // Operator overloads
    bool operator==(const User& other) const;
    bool operator!=(const User& other) const;
    friend std::ostream& operator<<(std::ostream& os, const User& user);

private:
    void logActivity(const std::string& message) const;
    void validateEmail(const std::string& email) const;
};

/**
 * AdminUser class with additional privileges
 */
class AdminUser : public User {
private:
    int adminLevel;
    std::vector<std::string> permissions;

public:
    // Constructors
    AdminUser(int userId, const std::string& userName, const std::string& userEmail, int level = 1);
    AdminUser(const User& user, int level = 1);
    
    // Methods
    void addPermission(const std::string& permission);
    bool hasPermission(const std::string& permission) const;
    void removePermission(const std::string& permission);
    std::unique_ptr<AdminUser> promoteUser(const User& user) const;
    
    // Accessors
    int getAdminLevel() const { return adminLevel; }
    const std::vector<std::string>& getPermissions() const { return permissions; }
    
    // Mutators
    void setAdminLevel(int level) { adminLevel = level; }
    
    // Override base class methods
    std::string toString() const override;
    bool isValid() const override;
    
    // Static methods
    static bool isValidAdminLevel(int level);
    static std::vector<std::string> getDefaultPermissions();

private:
    void initializeDefaultPermissions();
    bool isPermissionValid(const std::string& permission) const;
};

/**
 * UserManager class for managing users
 */
class UserManager {
private:
    std::vector<std::unique_ptr<User>> users;
    int nextUserId;

public:
    UserManager();
    ~UserManager() = default;
    
    // Non-copyable but movable
    UserManager(const UserManager&) = delete;
    UserManager& operator=(const UserManager&) = delete;
    UserManager(UserManager&&) = default;
    UserManager& operator=(UserManager&&) = default;
    
    // User management methods
    std::unique_ptr<User> createUser(const std::string& name, const std::string& email);
    User* findUser(int userId);
    const User* findUser(int userId) const;
    User* findUserByEmail(const std::string& email);
    bool deleteUser(int userId);
    
    // Bulk operations
    std::vector<User*> getAllUsers();
    std::vector<const User*> getAllUsers() const;
    std::vector<User*> searchUsers(const std::string& searchTerm);
    size_t getUserCount() const { return users.size(); }
    
    // Statistics
    double getAverageOrderValue() const;
    User* getMostActiveUser() const;
    
    // Template methods
    template<typename Predicate>
    std::vector<User*> findUsersIf(Predicate pred);
    
    template<typename Comparator>
    void sortUsers(Comparator comp);

private:
    int generateUserId();
    bool isEmailExists(const std::string& email) const;
    void validateUserData(const std::string& name, const std::string& email) const;
};

// Template method implementations
template<typename Predicate>
std::vector<User*> UserManager::findUsersIf(Predicate pred) {
    std::vector<User*> result;
    for (auto& user : users) {
        if (pred(*user)) {
            result.push_back(user.get());
        }
    }
    return result;
}

template<typename Comparator>
void UserManager::sortUsers(Comparator comp) {
    std::sort(users.begin(), users.end(), 
              [&comp](const std::unique_ptr<User>& a, const std::unique_ptr<User>& b) {
                  return comp(*a, *b);
              });
}

} // namespace Models

#endif // USER_H