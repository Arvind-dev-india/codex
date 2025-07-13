// Go Test Suite - Basic Structures and Functions
package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"regexp"
	"sync"
	"time"
)

// Constants
const (
	DefaultRole     = "user"
	MaxRetries      = 5
	MaxLoginAttempts = 3
	TimeoutDuration = 30 * time.Second
)

// Type definitions
type UserID int64
type UserRole string
type UserStatus int

// Enums using iota
const (
	StatusInactive UserStatus = iota
	StatusActive
	StatusPending
	StatusSuspended
)

// Interface definition
type UserRepository interface {
	FindByID(ctx context.Context, id UserID) (*User, error)
	Save(ctx context.Context, user *User) error
	Delete(ctx context.Context, id UserID) error
	FindByEmail(ctx context.Context, email string) (*User, error)
}

// Struct with embedded fields
type BaseEntity struct {
	ID        UserID    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`
}

// Main struct
type User struct {
	BaseEntity
	Name        string            `json:"name" db:"name" validate:"required,min=2"`
	Email       string            `json:"email" db:"email" validate:"required,email"`
	IsActive    bool              `json:"is_active" db:"is_active"`
	Role        UserRole          `json:"role" db:"role"`
	Status      UserStatus        `json:"status" db:"status"`
	Permissions []string          `json:"permissions" db:"permissions"`
	Metadata    map[string]interface{} `json:"metadata" db:"metadata"`
	mutex       sync.RWMutex      `json:"-"`
}

// Constructor function
func NewUser(id UserID, name, email string) (*User, error) {
	if name == "" {
		return nil, errors.New("name cannot be empty")
	}
	if !isValidEmail(email) {
		return nil, errors.New("invalid email format")
	}

	return &User{
		BaseEntity: BaseEntity{
			ID:        id,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		},
		Name:        name,
		Email:       email,
		IsActive:    true,
		Role:        DefaultRole,
		Status:      StatusActive,
		Permissions: make([]string, 0),
		Metadata:    make(map[string]interface{}),
	}, nil
}

// Method with receiver
func (u *User) UpdateEmail(newEmail string) error {
	if !isValidEmail(newEmail) {
		return errors.New("invalid email format")
	}
	
	u.mutex.Lock()
	defer u.mutex.Unlock()
	
	u.Email = newEmail
	u.UpdatedAt = time.Now()
	return nil
}

// Method with pointer receiver
func (u *User) AddPermission(permission string) {
	u.mutex.Lock()
	defer u.mutex.Unlock()
	
	// Check if permission already exists
	for _, p := range u.Permissions {
		if p == permission {
			return
		}
	}
	
	u.Permissions = append(u.Permissions, permission)
	u.UpdatedAt = time.Now()
}

// Method with value receiver
func (u User) GetDisplayName() string {
	return fmt.Sprintf("%s (%s)", u.Name, u.Email)
}

// Method returning multiple values
func (u *User) Validate() (bool, []string) {
	var errors []string
	
	if u.Name == "" {
		errors = append(errors, "name is required")
	}
	
	if !isValidEmail(u.Email) {
		errors = append(errors, "invalid email format")
	}
	
	return len(errors) == 0, errors
}

// Static-like function
func CreateGuestUser() *User {
	user, _ := NewUser(0, "Guest", "guest@example.com")
	user.IsActive = false
	user.Role = "guest"
	return user
}

// Generic-like function using interface{}
func GetMetadata(u *User, key string) (interface{}, bool) {
	u.mutex.RLock()
	defer u.mutex.RUnlock()
	
	value, exists := u.Metadata[key]
	return value, exists
}

// Function with context
func FetchUserAsync(ctx context.Context, repo UserRepository, id UserID) (*User, error) {
	// Create a channel for the result
	resultChan := make(chan *User, 1)
	errorChan := make(chan error, 1)
	
	go func() {
		user, err := repo.FindByID(ctx, id)
		if err != nil {
			errorChan <- err
			return
		}
		resultChan <- user
	}()
	
	select {
	case user := <-resultChan:
		return user, nil
	case err := <-errorChan:
		return nil, err
	case <-ctx.Done():
		return nil, ctx.Err()
	case <-time.After(TimeoutDuration):
		return nil, errors.New("operation timed out")
	}
}

// Variadic function
func ProcessUsers(users ...*User) []string {
	results := make([]string, 0, len(users))
	
	for _, user := range users {
		if user != nil && user.IsActive {
			results = append(results, user.GetDisplayName())
		}
	}
	
	return results
}

// Private helper function
func isValidEmail(email string) bool {
	emailRegex := regexp.MustCompile(`^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)
	return emailRegex.MatchString(email)
}

// Function with closure
func CreateUserValidator() func(*User) error {
	emailRegex := regexp.MustCompile(`^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)
	
	return func(user *User) error {
		if user == nil {
			return errors.New("user cannot be nil")
		}
		
		if user.Name == "" {
			return errors.New("name is required")
		}
		
		if !emailRegex.MatchString(user.Email) {
			return errors.New("invalid email format")
		}
		
		return nil
	}
}

// Main function
func main() {
	// Create a user
	user, err := NewUser(1, "John Doe", "john@example.com")
	if err != nil {
		log.Fatal(err)
	}
	
	// Add permissions
	user.AddPermission("read")
	user.AddPermission("write")
	
	// Validate user
	isValid, validationErrors := user.Validate()
	if !isValid {
		log.Printf("Validation errors: %v", validationErrors)
	}
	
	// Process users
	guest := CreateGuestUser()
	results := ProcessUsers(user, guest)
	
	fmt.Printf("Processed users: %v\n", results)
	fmt.Printf("User display name: %s\n", user.GetDisplayName())
}