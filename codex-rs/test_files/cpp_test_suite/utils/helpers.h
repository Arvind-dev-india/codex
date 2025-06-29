/**
 * Utility functions and helper classes for the C++ test suite
 */

#ifndef HELPERS_H
#define HELPERS_H

#include <string>
#include <vector>
#include <map>
#include <functional>
#include <memory>
#include <chrono>
#include <regex>
#include <algorithm>
#include <type_traits>
#include <optional>

namespace Utils {

/**
 * String utility functions
 */
class StringUtils {
public:
    static bool validateEmail(const std::string& email);
    static std::string generateHash(const std::string& text);
    static std::string formatCurrency(double amount, const std::string& currency = "USD");
    static double calculatePercentage(double part, double whole);
    static std::string trim(const std::string& str);
    static std::string toLower(const std::string& str);
    static std::string toUpper(const std::string& str);
    static std::vector<std::string> split(const std::string& str, char delimiter);
    static std::string join(const std::vector<std::string>& strings, const std::string& delimiter);
    static bool startsWith(const std::string& str, const std::string& prefix);
    static bool endsWith(const std::string& str, const std::string& suffix);
    static std::string replace(const std::string& str, const std::string& from, const std::string& to);
    
    // Template functions
    template<typename T>
    static std::string toString(const T& value);
    
    template<typename T>
    static std::optional<T> fromString(const std::string& str);

private:
    static const std::regex emailRegex;
};

// Template implementations
template<typename T>
std::string StringUtils::toString(const T& value) {
    if constexpr (std::is_arithmetic_v<T>) {
        return std::to_string(value);
    } else if constexpr (std::is_same_v<T, std::string>) {
        return value;
    } else if constexpr (requires { value.toString(); }) {
        return value.toString();
    } else if constexpr (requires { std::to_string(value); }) {
        return std::to_string(value);
    } else {
        return "Unknown";
    }
}

template<typename T>
std::optional<T> StringUtils::fromString(const std::string& str) {
    try {
        if constexpr (std::is_same_v<T, int>) {
            return std::stoi(str);
        } else if constexpr (std::is_same_v<T, long>) {
            return std::stol(str);
        } else if constexpr (std::is_same_v<T, float>) {
            return std::stof(str);
        } else if constexpr (std::is_same_v<T, double>) {
            return std::stod(str);
        } else if constexpr (std::is_same_v<T, std::string>) {
            return str;
        } else {
            return std::nullopt;
        }
    } catch (...) {
        return std::nullopt;
    }
}

/**
 * Timer class for measuring execution time
 */
class Timer {
private:
    std::string name;
    std::chrono::high_resolution_clock::time_point startTime;
    std::chrono::high_resolution_clock::time_point endTime;
    bool running;

public:
    explicit Timer(const std::string& timerName = "Operation");
    ~Timer();
    
    void start();
    void stop();
    void reset();
    
    std::chrono::milliseconds elapsed() const;
    double elapsedSeconds() const;
    
    const std::string& getName() const { return name; }
    bool isRunning() const { return running; }
    
    // RAII helper
    class ScopedTimer {
    private:
        Timer& timer;
    public:
        explicit ScopedTimer(Timer& t) : timer(t) { timer.start(); }
        ~ScopedTimer() { timer.stop(); }
    };
    
    ScopedTimer createScopedTimer() { return ScopedTimer(*this); }
};

/**
 * Data validator class
 */
class DataValidator {
public:
    struct ValidationResult {
        bool isValid;
        std::vector<std::string> errors;
        
        ValidationResult(bool valid = true) : isValid(valid) {}
        
        void addError(const std::string& error) {
            errors.push_back(error);
            isValid = false;
        }
        
        std::string getErrorsAsString() const {
            return StringUtils::join(errors, "; ");
        }
    };
    
    static ValidationResult validateUserData(const std::map<std::string, std::string>& data);
    static ValidationResult validateProductData(const std::map<std::string, std::string>& data);
    static ValidationResult validateEmail(const std::string& email);
    static ValidationResult validatePassword(const std::string& password);
    static ValidationResult validatePhoneNumber(const std::string& phone);
    
    // Template validation
    template<typename T>
    static ValidationResult validateRange(const T& value, const T& min, const T& max);
    
    template<typename T>
    static ValidationResult validateNotNull(const T* ptr);
    
    template<typename Container>
    static ValidationResult validateNotEmpty(const Container& container);

private:
    static bool isValidEmailFormat(const std::string& email);
    static bool isValidPasswordStrength(const std::string& password);
};

// Template implementations
template<typename T>
DataValidator::ValidationResult DataValidator::validateRange(const T& value, const T& min, const T& max) {
    ValidationResult result;
    if (value < min || value > max) {
        result.addError("Value " + StringUtils::toString(value) + 
                       " is not in range [" + StringUtils::toString(min) + 
                       ", " + StringUtils::toString(max) + "]");
    }
    return result;
}

template<typename T>
DataValidator::ValidationResult DataValidator::validateNotNull(const T* ptr) {
    ValidationResult result;
    if (ptr == nullptr) {
        result.addError("Pointer cannot be null");
    }
    return result;
}

template<typename Container>
DataValidator::ValidationResult DataValidator::validateNotEmpty(const Container& container) {
    ValidationResult result;
    if (container.empty()) {
        result.addError("Container cannot be empty");
    }
    return result;
}

/**
 * Configuration manager class
 */
class ConfigManager {
private:
    std::map<std::string, std::string> config;
    std::string configFile;

public:
    ConfigManager();
    explicit ConfigManager(const std::string& filename);
    
    // Configuration access
    std::string get(const std::string& key, const std::string& defaultValue = "") const;
    void set(const std::string& key, const std::string& value);
    bool has(const std::string& key) const;
    void remove(const std::string& key);
    
    // Type-safe getters
    template<typename T>
    T get(const std::string& key, const T& defaultValue = T{}) const;
    
    template<typename T>
    void set(const std::string& key, const T& value);
    
    // File operations
    bool loadFromFile(const std::string& filename);
    bool saveToFile(const std::string& filename = "") const;
    
    // Bulk operations
    void update(const std::map<std::string, std::string>& newConfig);
    std::map<std::string, std::string> getAll() const { return config; }
    void clear() { config.clear(); }
    
    // Utility
    size_t size() const { return config.size(); }
    bool empty() const { return config.empty(); }
    std::vector<std::string> getKeys() const;

private:
    void loadDefaults();
    std::string parseConfigLine(const std::string& line) const;
};

// Template implementations
template<typename T>
T ConfigManager::get(const std::string& key, const T& defaultValue) const {
    auto it = config.find(key);
    if (it == config.end()) {
        return defaultValue;
    }
    
    auto result = StringUtils::fromString<T>(it->second);
    return result.value_or(defaultValue);
}

template<typename T>
void ConfigManager::set(const std::string& key, const T& value) {
    config[key] = StringUtils::toString(value);
}

/**
 * Batch processor for handling collections
 */
template<typename T>
class BatchProcessor {
private:
    std::vector<T> items;
    size_t batchSize;
    std::function<void(const std::vector<T>&)> processor;

public:
    BatchProcessor(size_t size = 10) : batchSize(size) {}
    
    void setBatchSize(size_t size) { batchSize = size; }
    void setProcessor(std::function<void(const std::vector<T>&)> proc) { processor = proc; }
    
    void addItem(const T& item) {
        items.push_back(item);
        if (items.size() >= batchSize) {
            processBatch();
        }
    }
    
    void addItems(const std::vector<T>& newItems) {
        for (const auto& item : newItems) {
            addItem(item);
        }
    }
    
    void flush() {
        if (!items.empty()) {
            processBatch();
        }
    }
    
    size_t getPendingCount() const { return items.size(); }
    size_t getBatchSize() const { return batchSize; }

private:
    void processBatch() {
        if (processor && !items.empty()) {
            processor(items);
            items.clear();
        }
    }
};

/**
 * Deep merge utility for maps
 */
template<typename Key, typename Value>
std::map<Key, Value> deepMerge(const std::map<Key, Value>& map1, const std::map<Key, Value>& map2) {
    std::map<Key, Value> result = map1;
    
    for (const auto& pair : map2) {
        if constexpr (std::is_same_v<Value, std::map<Key, Value>>) {
            // Recursive merge for nested maps
            if (result.find(pair.first) != result.end()) {
                result[pair.first] = deepMerge(result[pair.first], pair.second);
            } else {
                result[pair.first] = pair.second;
            }
        } else {
            result[pair.first] = pair.second;
        }
    }
    
    return result;
}

/**
 * Singleton template class
 */
template<typename T>
class Singleton {
private:
    static std::unique_ptr<T> instance;
    static std::once_flag onceFlag;

protected:
    Singleton() = default;

public:
    virtual ~Singleton() = default;
    
    // Non-copyable and non-movable
    Singleton(const Singleton&) = delete;
    Singleton& operator=(const Singleton&) = delete;
    Singleton(Singleton&&) = delete;
    Singleton& operator=(Singleton&&) = delete;
    
    static T& getInstance() {
        std::call_once(onceFlag, []() {
            instance = std::make_unique<T>();
        });
        return *instance;
    }
    
    static void destroyInstance() {
        instance.reset();
    }
};

template<typename T>
std::unique_ptr<T> Singleton<T>::instance = nullptr;

template<typename T>
std::once_flag Singleton<T>::onceFlag;

/**
 * Logger class (singleton)
 */
class Logger : public Singleton<Logger> {
public:
    enum class Level {
        DEBUG,
        INFO,
        WARNING,
        ERROR,
        CRITICAL
    };

private:
    std::vector<std::string> logs;
    Level currentLevel;
    bool enableConsoleOutput;
    std::string logFile;

public:
    Logger();
    
    void log(const std::string& message, Level level = Level::INFO);
    void debug(const std::string& message) { log(message, Level::DEBUG); }
    void info(const std::string& message) { log(message, Level::INFO); }
    void warning(const std::string& message) { log(message, Level::WARNING); }
    void error(const std::string& message) { log(message, Level::ERROR); }
    void critical(const std::string& message) { log(message, Level::CRITICAL); }
    
    // Template logging
    template<typename... Args>
    void logf(Level level, const std::string& format, Args&&... args);
    
    // Configuration
    void setLevel(Level level) { currentLevel = level; }
    void enableConsole(bool enable) { enableConsoleOutput = enable; }
    void setLogFile(const std::string& filename) { logFile = filename; }
    
    // Access
    std::vector<std::string> getLogs() const { return logs; }
    void clearLogs() { logs.clear(); }
    size_t getLogCount() const { return logs.size(); }
    
    // Utility
    static std::string levelToString(Level level);
    static Level stringToLevel(const std::string& levelStr);

private:
    std::string formatLogMessage(const std::string& message, Level level) const;
    bool shouldLog(Level level) const;
    void writeToFile(const std::string& message) const;
};

// Template implementation
template<typename... Args>
void Logger::logf(Level level, const std::string& format, Args&&... args) {
    // Simple format implementation - in real code you'd use a proper formatting library
    std::string message = format;
    // This is a simplified implementation
    log(message, level);
}

/**
 * Retry utility with exponential backoff
 */
template<typename Func>
class RetryPolicy {
private:
    Func function;
    int maxAttempts;
    std::chrono::milliseconds initialDelay;
    double backoffMultiplier;

public:
    RetryPolicy(Func func, int attempts = 3, 
                std::chrono::milliseconds delay = std::chrono::milliseconds(100),
                double multiplier = 2.0)
        : function(func), maxAttempts(attempts), initialDelay(delay), backoffMultiplier(multiplier) {}
    
    template<typename... Args>
    auto execute(Args&&... args) -> decltype(function(std::forward<Args>(args)...)) {
        std::chrono::milliseconds currentDelay = initialDelay;
        
        for (int attempt = 1; attempt <= maxAttempts; ++attempt) {
            try {
                return function(std::forward<Args>(args)...);
            } catch (const std::exception& e) {
                if (attempt == maxAttempts) {
                    throw; // Re-throw on final attempt
                }
                
                Logger::getInstance().warning("Attempt " + std::to_string(attempt) + 
                                             " failed: " + e.what() + 
                                             ". Retrying in " + std::to_string(currentDelay.count()) + "ms");
                
                std::this_thread::sleep_for(currentDelay);
                currentDelay = std::chrono::milliseconds(
                    static_cast<long>(currentDelay.count() * backoffMultiplier));
            }
        }
        
        // Should never reach here
        throw std::runtime_error("Retry policy failed unexpectedly");
    }
};

// Helper function to create retry policy
template<typename Func>
RetryPolicy<Func> createRetryPolicy(Func func, int maxAttempts = 3) {
    return RetryPolicy<Func>(func, maxAttempts);
}

} // namespace Utils

#endif // HELPERS_H