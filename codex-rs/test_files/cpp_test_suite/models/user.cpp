/**
 * User model implementation for the C++ test suite
 */

#include "user.h"
#include <algorithm>
#include <sstream>
#include <stdexcept>
#include <iostream>
#include <regex>

namespace Models {

// User class implementation

User::User() 
    : id(0), name(""), email(""), createdAt(std::chrono::system_clock::now()) {
}

User::User(int userId, const std::string& userName, const std::string& userEmail)
    : id(userId), name(userName), email(userEmail), createdAt(std::chrono::system_clock::now()) {
    validateEmail(email);
}

User::User(const User& other)
    : id(other.id), name(other.name), email(other.email), 
      createdAt(other.createdAt), orders(other.orders) {
}

User::User(User&& other) noexcept
    : id(other.id), name(std::move(other.name)), email(std::move(other.email)),
      createdAt(other.createdAt), orders(std::move(other.orders)) {
    other.id = 0;
}

User& User::operator=(const User& other) {
    if (this != &other) {
        id = other.id;
        name = other.name;
        email = other.email;
        createdAt = other.createdAt;
        orders = other.orders;
    }
    return *this;
}

User& User::operator=(User&& other) noexcept {
    if (this != &other) {
        id = other.id;
        name = std::move(other.name);
        email = std::move(other.email);
        createdAt = other.createdAt;
        orders = std::move(other.orders);
        other.id = 0;
    }
    return *this;
}

void User::addOrder(std::shared_ptr<Order> order) {
    if (!order) {
        throw std::invalid_argument("Order cannot be null");
    }
    
    order->setUserId(id);
    orders.push_back(order);
    logActivity("Order " + std::to_string(order->getId()) + " added to user " + name);
}

std::shared_ptr<Order> User::getOrder(int orderId) const {
    auto it = std::find_if(orders.begin(), orders.end(),
                          [orderId](const std::shared_ptr<Order>& order) {
                              return order->getId() == orderId;
                          });
    return (it != orders.end()) ? *it : nullptr;
}

double User::getTotalOrderValue() const {
    double total = 0.0;
    for (const auto& order : orders) {
        total += order->calculateTotal();
    }
    return total;
}

size_t User::getOrderCount() const {
    return orders.size();
}

std::string User::toString() const {
    std::ostringstream oss;
    oss << "User: " << name << " (" << email << ") - " << orders.size() << " orders";
    return oss.str();
}

bool User::isValid() const {
    return id > 0 && !name.empty() && !email.empty();
}

bool User::operator==(const User& other) const {
    return id == other.id && name == other.name && email == other.email;
}

bool User::operator!=(const User& other) const {
    return !(*this == other);
}

std::ostream& operator<<(std::ostream& os, const User& user) {
    os << user.toString();
    return os;
}

void User::logActivity(const std::string& message) const {
    auto now = std::chrono::system_clock::now();
    auto time_t = std::chrono::system_clock::to_time_t(now);
    std::cout << "[" << std::ctime(&time_t) << "] User Activity: " << message << std::endl;
}

void User::validateEmail(const std::string& email) const {
    std::regex emailRegex(R"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})");
    if (!std::regex_match(email, emailRegex)) {
        throw std::invalid_argument("Invalid email format: " + email);
    }
}

// AdminUser class implementation

AdminUser::AdminUser(int userId, const std::string& userName, const std::string& userEmail, int level)
    : User(userId, userName, userEmail), adminLevel(level) {
    if (!isValidAdminLevel(level)) {
        throw std::invalid_argument("Invalid admin level: " + std::to_string(level));
    }
    initializeDefaultPermissions();
}

AdminUser::AdminUser(const User& user, int level)
    : User(user), adminLevel(level) {
    if (!isValidAdminLevel(level)) {
        throw std::invalid_argument("Invalid admin level: " + std::to_string(level));
    }
    initializeDefaultPermissions();
}

void AdminUser::addPermission(const std::string& permission) {
    if (!isPermissionValid(permission)) {
        throw std::invalid_argument("Invalid permission: " + permission);
    }
    
    if (std::find(permissions.begin(), permissions.end(), permission) == permissions.end()) {
        permissions.push_back(permission);
        logActivity("Permission '" + permission + "' added");
    }
}

bool AdminUser::hasPermission(const std::string& permission) const {
    return std::find(permissions.begin(), permissions.end(), permission) != permissions.end();
}

void AdminUser::removePermission(const std::string& permission) {
    auto it = std::find(permissions.begin(), permissions.end(), permission);
    if (it != permissions.end()) {
        permissions.erase(it);
        logActivity("Permission '" + permission + "' removed");
    }
}

std::unique_ptr<AdminUser> AdminUser::promoteUser(const User& user) const {
    return std::make_unique<AdminUser>(user, 1);
}

std::string AdminUser::toString() const {
    std::ostringstream oss;
    oss << User::toString() << " [Admin Level " << adminLevel << ", " 
        << permissions.size() << " permissions]";
    return oss.str();
}

bool AdminUser::isValid() const {
    return User::isValid() && isValidAdminLevel(adminLevel);
}

bool AdminUser::isValidAdminLevel(int level) {
    return level >= 1 && level <= 5;
}

std::vector<std::string> AdminUser::getDefaultPermissions() {
    return {"read_users", "read_orders", "basic_admin"};
}

void AdminUser::initializeDefaultPermissions() {
    permissions = getDefaultPermissions();
    if (adminLevel >= 3) {
        permissions.push_back("manage_users");
    }
    if (adminLevel >= 4) {
        permissions.push_back("manage_orders");
    }
    if (adminLevel >= 5) {
        permissions.push_back("system_admin");
    }
}

bool AdminUser::isPermissionValid(const std::string& permission) const {
    // Simple validation - in real system this would be more sophisticated
    return !permission.empty() && permission.length() <= 50;
}

// UserManager class implementation

UserManager::UserManager() : nextUserId(1) {
}

std::unique_ptr<User> UserManager::createUser(const std::string& name, const std::string& email) {
    validateUserData(name, email);
    
    if (isEmailExists(email)) {
        throw std::runtime_error("User with email " + email + " already exists");
    }
    
    auto user = std::make_unique<User>(generateUserId(), name, email);
    User* userPtr = user.get();
    users.push_back(std::move(user));
    
    std::cout << "User created: " << name << " (" << email << ")" << std::endl;
    return std::make_unique<User>(*userPtr); // Return a copy
}

User* UserManager::findUser(int userId) {
    auto it = std::find_if(users.begin(), users.end(),
                          [userId](const std::unique_ptr<User>& user) {
                              return user->getId() == userId;
                          });
    return (it != users.end()) ? it->get() : nullptr;
}

const User* UserManager::findUser(int userId) const {
    auto it = std::find_if(users.begin(), users.end(),
                          [userId](const std::unique_ptr<User>& user) {
                              return user->getId() == userId;
                          });
    return (it != users.end()) ? it->get() : nullptr;
}

User* UserManager::findUserByEmail(const std::string& email) {
    auto it = std::find_if(users.begin(), users.end(),
                          [&email](const std::unique_ptr<User>& user) {
                              return user->getEmail() == email;
                          });
    return (it != users.end()) ? it->get() : nullptr;
}

bool UserManager::deleteUser(int userId) {
    auto it = std::find_if(users.begin(), users.end(),
                          [userId](const std::unique_ptr<User>& user) {
                              return user->getId() == userId;
                          });
    
    if (it != users.end()) {
        std::cout << "User deleted: " << (*it)->getName() << std::endl;
        users.erase(it);
        return true;
    }
    return false;
}

std::vector<User*> UserManager::getAllUsers() {
    std::vector<User*> result;
    result.reserve(users.size());
    for (auto& user : users) {
        result.push_back(user.get());
    }
    return result;
}

std::vector<const User*> UserManager::getAllUsers() const {
    std::vector<const User*> result;
    result.reserve(users.size());
    for (const auto& user : users) {
        result.push_back(user.get());
    }
    return result;
}

std::vector<User*> UserManager::searchUsers(const std::string& searchTerm) {
    std::vector<User*> result;
    std::string lowerSearchTerm = searchTerm;
    std::transform(lowerSearchTerm.begin(), lowerSearchTerm.end(), 
                   lowerSearchTerm.begin(), ::tolower);
    
    for (auto& user : users) {
        std::string lowerName = user->getName();
        std::string lowerEmail = user->getEmail();
        std::transform(lowerName.begin(), lowerName.end(), lowerName.begin(), ::tolower);
        std::transform(lowerEmail.begin(), lowerEmail.end(), lowerEmail.begin(), ::tolower);
        
        if (lowerName.find(lowerSearchTerm) != std::string::npos ||
            lowerEmail.find(lowerSearchTerm) != std::string::npos) {
            result.push_back(user.get());
        }
    }
    return result;
}

double UserManager::getAverageOrderValue() const {
    if (users.empty()) return 0.0;
    
    double totalValue = 0.0;
    size_t totalOrders = 0;
    
    for (const auto& user : users) {
        totalValue += user->getTotalOrderValue();
        totalOrders += user->getOrderCount();
    }
    
    return totalOrders > 0 ? totalValue / totalOrders : 0.0;
}

User* UserManager::getMostActiveUser() const {
    if (users.empty()) return nullptr;
    
    auto maxIt = std::max_element(users.begin(), users.end(),
                                 [](const std::unique_ptr<User>& a, const std::unique_ptr<User>& b) {
                                     return a->getOrderCount() < b->getOrderCount();
                                 });
    
    return maxIt->get();
}

int UserManager::generateUserId() {
    return nextUserId++;
}

bool UserManager::isEmailExists(const std::string& email) const {
    return std::any_of(users.begin(), users.end(),
                      [&email](const std::unique_ptr<User>& user) {
                          return user->getEmail() == email;
                      });
}

void UserManager::validateUserData(const std::string& name, const std::string& email) const {
    if (name.empty()) {
        throw std::invalid_argument("Name cannot be empty");
    }
    if (email.empty()) {
        throw std::invalid_argument("Email cannot be empty");
    }
    if (name.length() > 100) {
        throw std::invalid_argument("Name too long");
    }
    if (email.length() > 100) {
        throw std::invalid_argument("Email too long");
    }
}

} // namespace Models