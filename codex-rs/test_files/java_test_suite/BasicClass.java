// Java Test Suite - Basic Class
package com.example.test;

import java.util.*;
import java.util.concurrent.CompletableFuture;

/**
 * Basic User class demonstrating Java features
 */
public class BasicClass {
    // Static constants
    public static final String DEFAULT_ROLE = "USER";
    private static final int MAX_LOGIN_ATTEMPTS = 3;
    
    // Instance fields
    private final Long id;
    private String name;
    private String email;
    private boolean isActive;
    private List<String> permissions;
    private Map<String, Object> metadata;
    
    // Constructor
    public BasicClass(Long id, String name, String email) {
        this.id = Objects.requireNonNull(id, "ID cannot be null");
        this.name = Objects.requireNonNull(name, "Name cannot be null");
        this.email = Objects.requireNonNull(email, "Email cannot be null");
        this.isActive = true;
        this.permissions = new ArrayList<>();
        this.metadata = new HashMap<>();
    }
    
    // Getter methods
    public Long getId() {
        return id;
    }
    
    public String getName() {
        return name;
    }
    
    public String getEmail() {
        return email;
    }
    
    public boolean isActive() {
        return isActive;
    }
    
    // Setter methods
    public void setName(String name) {
        this.name = Objects.requireNonNull(name, "Name cannot be null");
    }
    
    public void setEmail(String email) {
        if (validateEmail(email)) {
            this.email = email;
        } else {
            throw new IllegalArgumentException("Invalid email format");
        }
    }
    
    public void setActive(boolean active) {
        this.isActive = active;
    }
    
    // Business methods
    public boolean updateEmail(String newEmail) {
        try {
            setEmail(newEmail);
            return true;
        } catch (IllegalArgumentException e) {
            return false;
        }
    }
    
    // Private validation method
    private boolean validateEmail(String email) {
        return email != null && 
               email.contains("@") && 
               email.contains(".") &&
               email.length() > 5;
    }
    
    // Static factory method
    public static BasicClass createGuest() {
        BasicClass guest = new BasicClass(0L, "Guest", "guest@example.com");
        guest.setActive(false);
        return guest;
    }
    
    // Generic method
    public <T> Optional<T> getMetadata(String key, Class<T> type) {
        Object value = metadata.get(key);
        if (type.isInstance(value)) {
            return Optional.of(type.cast(value));
        }
        return Optional.empty();
    }
    
    // Method with varargs
    public void addPermissions(String... permissions) {
        this.permissions.addAll(Arrays.asList(permissions));
    }
    
    // Async method
    public CompletableFuture<String> fetchUserDataAsync() {
        return CompletableFuture.supplyAsync(() -> {
            // Simulate async operation
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
            return "User data for " + name;
        });
    }
    
    // Override methods
    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        BasicClass that = (BasicClass) obj;
        return Objects.equals(id, that.id);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
    
    @Override
    public String toString() {
        return String.format("BasicClass{id=%d, name='%s', email='%s', active=%s}", 
                           id, name, email, isActive);
    }
    
    // Inner class
    public static class UserBuilder {
        private Long id;
        private String name;
        private String email;
        
        public UserBuilder setId(Long id) {
            this.id = id;
            return this;
        }
        
        public UserBuilder setName(String name) {
            this.name = name;
            return this;
        }
        
        public UserBuilder setEmail(String email) {
            this.email = email;
            return this;
        }
        
        public BasicClass build() {
            return new BasicClass(id, name, email);
        }
    }
}