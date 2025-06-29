/**
 * Repository pattern implementation for data access
 */

#ifndef REPOSITORY_H
#define REPOSITORY_H

#include <memory>
#include <vector>
#include <map>
#include <functional>
#include <optional>
#include <type_traits>

namespace Data {

/**
 * Abstract base repository interface
 */
template<typename T>
class Repository {
public:
    virtual ~Repository() = default;
    
    // Pure virtual methods
    virtual std::unique_ptr<T> add(std::unique_ptr<T> entity) = 0;
    virtual T* getById(int id) = 0;
    virtual const T* getById(int id) const = 0;
    virtual std::vector<T*> getAll() = 0;
    virtual std::vector<const T*> getAll() const = 0;
    virtual bool update(const T& entity) = 0;
    virtual bool remove(int id) = 0;
    
    // Optional virtual methods with default implementations
    virtual size_t count() const {
        return getAll().size();
    }
    
    virtual bool exists(int id) const {
        return getById(id) != nullptr;
    }
    
    virtual void clear() {
        auto all = getAll();
        for (const auto* entity : all) {
            if constexpr (requires { entity->getId(); }) {
                remove(entity->getId());
            }
        }
    }
    
    // Template methods
    template<typename Predicate>
    std::vector<T*> findIf(Predicate pred) {
        std::vector<T*> result;
        auto all = getAll();
        std::copy_if(all.begin(), all.end(), std::back_inserter(result), pred);
        return result;
    }
    
    template<typename Predicate>
    std::vector<const T*> findIf(Predicate pred) const {
        std::vector<const T*> result;
        auto all = getAll();
        std::copy_if(all.begin(), all.end(), std::back_inserter(result), pred);
        return result;
    }
    
    template<typename Predicate>
    T* findFirst(Predicate pred) {
        auto all = getAll();
        auto it = std::find_if(all.begin(), all.end(), pred);
        return (it != all.end()) ? *it : nullptr;
    }
    
    template<typename Predicate>
    const T* findFirst(Predicate pred) const {
        auto all = getAll();
        auto it = std::find_if(all.begin(), all.end(), pred);
        return (it != all.end()) ? *it : nullptr;
    }
};

/**
 * In-memory repository implementation
 */
template<typename T>
class InMemoryRepository : public Repository<T> {
private:
    std::map<int, std::unique_ptr<T>> data;
    int nextId;
    mutable std::string lastOperation;

public:
    InMemoryRepository() : nextId(1) {}
    
    // Virtual destructor
    virtual ~InMemoryRepository() = default;
    
    // Non-copyable but movable
    InMemoryRepository(const InMemoryRepository&) = delete;
    InMemoryRepository& operator=(const InMemoryRepository&) = delete;
    InMemoryRepository(InMemoryRepository&&) = default;
    InMemoryRepository& operator=(InMemoryRepository&&) = default;
    
    // Repository interface implementation
    std::unique_ptr<T> add(std::unique_ptr<T> entity) override {
        if (!entity) {
            throw std::invalid_argument("Entity cannot be null");
        }
        
        // Assign ID if entity doesn't have one or has invalid ID
        int entityId;
        if constexpr (requires { entity->getId(); entity->setId(1); }) {
            entityId = entity->getId();
            if (entityId <= 0) {
                entityId = nextId++;
                entity->setId(entityId);
            } else {
                nextId = std::max(nextId, entityId + 1);
            }
        } else {
            entityId = nextId++;
        }
        
        T* entityPtr = entity.get();
        data[entityId] = std::move(entity);
        logOperation("ADD", entityId);
        
        // Return a copy of the added entity
        if constexpr (std::is_copy_constructible_v<T>) {
            return std::make_unique<T>(*entityPtr);
        } else {
            return nullptr; // Can't return copy if not copy constructible
        }
    }
    
    T* getById(int id) override {
        auto it = data.find(id);
        if (it != data.end()) {
            logOperation("GET", id);
            return it->second.get();
        }
        return nullptr;
    }
    
    const T* getById(int id) const override {
        auto it = data.find(id);
        if (it != data.end()) {
            logOperation("GET", id);
            return it->second.get();
        }
        return nullptr;
    }
    
    std::vector<T*> getAll() override {
        std::vector<T*> result;
        result.reserve(data.size());
        for (auto& pair : data) {
            result.push_back(pair.second.get());
        }
        logOperation("GET_ALL", static_cast<int>(result.size()));
        return result;
    }
    
    std::vector<const T*> getAll() const override {
        std::vector<const T*> result;
        result.reserve(data.size());
        for (const auto& pair : data) {
            result.push_back(pair.second.get());
        }
        logOperation("GET_ALL", static_cast<int>(result.size()));
        return result;
    }
    
    bool update(const T& entity) override {
        int entityId;
        if constexpr (requires { entity.getId(); }) {
            entityId = entity.getId();
        } else {
            return false; // Can't update without ID
        }
        
        auto it = data.find(entityId);
        if (it != data.end()) {
            // Create a copy of the entity
            if constexpr (std::is_copy_constructible_v<T>) {
                it->second = std::make_unique<T>(entity);
                logOperation("UPDATE", entityId);
                return true;
            }
        }
        return false;
    }
    
    bool remove(int id) override {
        auto it = data.find(id);
        if (it != data.end()) {
            data.erase(it);
            logOperation("DELETE", id);
            return true;
        }
        return false;
    }
    
    // Additional methods specific to InMemoryRepository
    size_t count() const override {
        return data.size();
    }
    
    bool exists(int id) const override {
        return data.find(id) != data.end();
    }
    
    void clear() override {
        size_t count = data.size();
        data.clear();
        nextId = 1;
        logOperation("CLEAR", static_cast<int>(count));
    }
    
    // Utility methods
    const std::string& getLastOperation() const {
        return lastOperation;
    }
    
    std::vector<int> getAllIds() const {
        std::vector<int> ids;
        ids.reserve(data.size());
        for (const auto& pair : data) {
            ids.push_back(pair.first);
        }
        return ids;
    }
    
    // Statistics
    int getNextId() const { return nextId; }
    bool isEmpty() const { return data.empty(); }

private:
    void logOperation(const std::string& operation, int identifier) const {
        lastOperation = operation + ":" + std::to_string(identifier);
        // In a real implementation, this might log to a file or console
    }
};

/**
 * Specialized repository for entities with specific requirements
 */
template<typename T>
class CachedRepository : public Repository<T> {
private:
    std::unique_ptr<Repository<T>> baseRepository;
    mutable std::map<int, std::unique_ptr<T>> cache;
    mutable bool cacheValid;
    size_t maxCacheSize;

public:
    explicit CachedRepository(std::unique_ptr<Repository<T>> baseRepo, size_t maxCache = 100)
        : baseRepository(std::move(baseRepo)), cacheValid(false), maxCacheSize(maxCache) {
        if (!baseRepository) {
            throw std::invalid_argument("Base repository cannot be null");
        }
    }
    
    // Repository interface implementation
    std::unique_ptr<T> add(std::unique_ptr<T> entity) override {
        invalidateCache();
        return baseRepository->add(std::move(entity));
    }
    
    T* getById(int id) override {
        // Check cache first
        if (cacheValid) {
            auto it = cache.find(id);
            if (it != cache.end()) {
                return it->second.get();
            }
        }
        
        // Get from base repository
        T* entity = baseRepository->getById(id);
        if (entity && cache.size() < maxCacheSize) {
            // Add to cache
            if constexpr (std::is_copy_constructible_v<T>) {
                cache[id] = std::make_unique<T>(*entity);
            }
        }
        
        return entity;
    }
    
    const T* getById(int id) const override {
        return const_cast<CachedRepository*>(this)->getById(id);
    }
    
    std::vector<T*> getAll() override {
        return baseRepository->getAll();
    }
    
    std::vector<const T*> getAll() const override {
        return baseRepository->getAll();
    }
    
    bool update(const T& entity) override {
        invalidateCache();
        return baseRepository->update(entity);
    }
    
    bool remove(int id) override {
        invalidateCache();
        return baseRepository->remove(id);
    }
    
    // Cache management
    void clearCache() {
        cache.clear();
        cacheValid = false;
    }
    
    void warmCache() {
        clearCache();
        auto all = baseRepository->getAll();
        for (T* entity : all) {
            if (cache.size() >= maxCacheSize) break;
            
            if constexpr (requires { entity->getId(); }) {
                int id = entity->getId();
                if constexpr (std::is_copy_constructible_v<T>) {
                    cache[id] = std::make_unique<T>(*entity);
                }
            }
        }
        cacheValid = true;
    }
    
    // Statistics
    size_t getCacheSize() const { return cache.size(); }
    size_t getMaxCacheSize() const { return maxCacheSize; }
    bool isCacheValid() const { return cacheValid; }

private:
    void invalidateCache() {
        cache.clear();
        cacheValid = false;
    }
};

/**
 * Repository factory for creating different types of repositories
 */
class RepositoryFactory {
public:
    template<typename T>
    static std::unique_ptr<Repository<T>> createInMemoryRepository() {
        return std::make_unique<InMemoryRepository<T>>();
    }
    
    template<typename T>
    static std::unique_ptr<Repository<T>> createCachedRepository(
        std::unique_ptr<Repository<T>> baseRepo, size_t maxCacheSize = 100) {
        return std::make_unique<CachedRepository<T>>(std::move(baseRepo), maxCacheSize);
    }
    
    template<typename T>
    static std::unique_ptr<Repository<T>> createCachedInMemoryRepository(size_t maxCacheSize = 100) {
        auto baseRepo = createInMemoryRepository<T>();
        return createCachedRepository<T>(std::move(baseRepo), maxCacheSize);
    }
};

/**
 * Repository configuration
 */
struct RepositoryConfig {
    bool enableCaching = false;
    size_t maxCacheSize = 100;
    bool enableLogging = true;
    std::string logLevel = "INFO";
    
    static RepositoryConfig getDefault();
    static RepositoryConfig getTestConfig();
    
    bool isValid() const;
    std::string toString() const;
};

/**
 * Unit of Work pattern for managing multiple repository operations
 */
class UnitOfWork {
private:
    std::vector<std::function<void()>> operations;
    bool committed;

public:
    UnitOfWork() : committed(false) {}
    
    ~UnitOfWork() {
        if (!committed) {
            rollback();
        }
    }
    
    // Non-copyable and non-movable
    UnitOfWork(const UnitOfWork&) = delete;
    UnitOfWork& operator=(const UnitOfWork&) = delete;
    UnitOfWork(UnitOfWork&&) = delete;
    UnitOfWork& operator=(UnitOfWork&&) = delete;
    
    template<typename T>
    void registerAdd(Repository<T>& repo, std::unique_ptr<T> entity) {
        operations.push_back([&repo, entity = std::move(entity)]() mutable {
            repo.add(std::move(entity));
        });
    }
    
    template<typename T>
    void registerUpdate(Repository<T>& repo, const T& entity) {
        operations.push_back([&repo, &entity]() {
            repo.update(entity);
        });
    }
    
    template<typename T>
    void registerRemove(Repository<T>& repo, int id) {
        operations.push_back([&repo, id]() {
            repo.remove(id);
        });
    }
    
    void commit() {
        for (auto& operation : operations) {
            operation();
        }
        committed = true;
        operations.clear();
    }
    
    void rollback() {
        operations.clear();
        committed = false;
    }
    
    size_t getOperationCount() const { return operations.size(); }
    bool isCommitted() const { return committed; }
};

} // namespace Data

#endif // REPOSITORY_H