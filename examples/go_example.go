// Go example demonstrating various patterns for code search

package main

import (
	"encoding/json"
	"fmt"
	"log"
	"regexp"
	"sort"
	"strings"
	"time"
)

// User represents a user in the system
type User struct {
	ID        int       `json:"id"`
	Name      string    `json:"name"`
	Email     string    `json:"email"`
	Role      string    `json:"role"`
	CreatedAt time.Time `json:"created_at"`
	IsActive  bool      `json:"is_active"`
}

// CreateUserRequest represents the request to create a new user
type CreateUserRequest struct {
	Name  string `json:"name"`
	Email string `json:"email"`
	Role  string `json:"role"`
}

// UserService handles user operations
type UserService struct {
	users  map[int]*User
	nextID int
}

// NewUserService creates a new UserService instance
func NewUserService() *UserService {
	return &UserService{
		users:  make(map[int]*User),
		nextID: 1,
	}
}

// CreateUser creates a new user
func (s *UserService) CreateUser(req CreateUserRequest) (*User, error) {
	if err := s.validateUserRequest(req); err != nil {
		return nil, err
	}

	user := &User{
		ID:        s.nextID,
		Name:      req.Name,
		Email:     req.Email,
		Role:      req.Role,
		CreatedAt: time.Now(),
		IsActive:  true,
	}

	s.users[s.nextID] = user
	s.nextID++

	log.Printf("Created user: %s", user.Name)
	return user, nil
}

// GetUserByID retrieves a user by ID
func (s *UserService) GetUserByID(id int) (*User, error) {
	user, exists := s.users[id]
	if !exists {
		return nil, fmt.Errorf("user with ID %d not found", id)
	}
	return user, nil
}

// GetAllUsers returns all users
func (s *UserService) GetAllUsers() []*User {
	users := make([]*User, 0, len(s.users))
	for _, user := range s.users {
		users = append(users, user)
	}
	return users
}

// UpdateUser updates an existing user
func (s *UserService) UpdateUser(id int, updates map[string]interface{}) (*User, error) {
	user, exists := s.users[id]
	if !exists {
		return nil, fmt.Errorf("user with ID %d not found", id)
	}

	// Update fields if provided
	if name, ok := updates["name"].(string); ok {
		user.Name = name
	}
	if email, ok := updates["email"].(string); ok {
		if err := s.validateEmail(email); err != nil {
			return nil, err
		}
		user.Email = email
	}
	if role, ok := updates["role"].(string); ok {
		user.Role = role
	}
	if isActive, ok := updates["is_active"].(bool); ok {
		user.IsActive = isActive
	}

	log.Printf("Updated user: %s", user.Name)
	return user, nil
}

// DeleteUser deletes a user by ID
func (s *UserService) DeleteUser(id int) error {
	user, exists := s.users[id]
	if !exists {
		return fmt.Errorf("user with ID %d not found", id)
	}

	delete(s.users, id)
	log.Printf("Deleted user: %s", user.Name)
	return nil
}

// SearchUsers searches users by name or email
func (s *UserService) SearchUsers(query string) []*User {
	var results []*User
	query = strings.ToLower(query)

	for _, user := range s.users {
		if strings.Contains(strings.ToLower(user.Name), query) ||
			strings.Contains(strings.ToLower(user.Email), query) {
			results = append(results, user)
		}
	}

	return results
}

// GetUsersByRole returns users filtered by role
func (s *UserService) GetUsersByRole(role string) []*User {
	var results []*User
	for _, user := range s.users {
		if user.Role == role {
			results = append(results, user)
		}
	}
	return results
}

// validateUserRequest validates user creation request
func (s *UserService) validateUserRequest(req CreateUserRequest) error {
	if strings.TrimSpace(req.Name) == "" {
		return fmt.Errorf("name is required")
	}
	if strings.TrimSpace(req.Email) == "" {
		return fmt.Errorf("email is required")
	}
	if err := s.validateEmail(req.Email); err != nil {
		return err
	}
	if req.Role == "" {
		req.Role = "user" // default role
	}
	return nil
}

// validateEmail validates email format
func (s *UserService) validateEmail(email string) error {
	emailRegex := regexp.MustCompile(`^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)
	if !emailRegex.MatchString(email) {
		return fmt.Errorf("invalid email format")
	}
	return nil
}

// ToJSON converts user to JSON string
func (u *User) ToJSON() (string, error) {
	data, err := json.Marshal(u)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// SortUsersByName sorts users by name
func SortUsersByName(users []*User) {
	sort.Slice(users, func(i, j int) bool {
		return users[i].Name < users[j].Name
	})
}

// FilterActiveUsers returns only active users
func FilterActiveUsers(users []*User) []*User {
	var active []*User
	for _, user := range users {
		if user.IsActive {
			active = append(active, user)
		}
	}
	return active
}

// Main function demonstrating the user service
func main() {
	service := NewUserService()

	// Create some users
	user1, err := service.CreateUser(CreateUserRequest{
		Name:  "Alice Johnson",
		Email: "alice@example.com",
		Role:  "admin",
	})
	if err != nil {
		log.Fatal(err)
	}

	user2, err := service.CreateUser(CreateUserRequest{
		Name:  "Bob Smith",
		Email: "bob@example.com",
		Role:  "user",
	})
	if err != nil {
		log.Fatal(err)
	}

	user3, err := service.CreateUser(CreateUserRequest{
		Name:  "Charlie Brown",
		Email: "charlie@example.com",
		Role:  "moderator",
	})
	if err != nil {
		log.Fatal(err)
	}

	// Display all users
	fmt.Println("All users:")
	allUsers := service.GetAllUsers()
	SortUsersByName(allUsers)
	for _, user := range allUsers {
		fmt.Printf("  %d: %s (%s) - %s\n", user.ID, user.Name, user.Email, user.Role)
	}

	// Search users
	fmt.Println("\nSearch results for 'Alice':")
	searchResults := service.SearchUsers("Alice")
	for _, user := range searchResults {
		fmt.Printf("  %s (%s)\n", user.Name, user.Email)
	}

	// Get users by role
	fmt.Println("\nAdmin users:")
	adminUsers := service.GetUsersByRole("admin")
	for _, user := range adminUsers {
		fmt.Printf("  %s (%s)\n", user.Name, user.Email)
	}

	// Update a user
	updatedUser, err := service.UpdateUser(user2.ID, map[string]interface{}{
		"role": "moderator",
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("\nUpdated user: %s is now a %s\n", updatedUser.Name, updatedUser.Role)

	// Convert to JSON
	jsonData, err := user1.ToJSON()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("\nUser JSON: %s\n", jsonData)
}
