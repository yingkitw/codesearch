// Java example demonstrating various patterns for code search

import java.util.*;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

/**
 * User class representing a user in the system
 */
public class User {
    private int id;
    private String name;
    private String email;
    private String role;
    private LocalDateTime createdAt;
    private boolean isActive;

    // Constructor
    public User(int id, String name, String email, String role) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.role = role;
        this.createdAt = LocalDateTime.now();
        this.isActive = true;
    }

    // Getters and setters
    public int getId() { return id; }
    public void setId(int id) { this.id = id; }

    public String getName() { return name; }
    public void setName(String name) { this.name = name; }

    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }

    public String getRole() { return role; }
    public void setRole(String role) { this.role = role; }

    public LocalDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(LocalDateTime createdAt) { this.createdAt = createdAt; }

    public boolean isActive() { return isActive; }
    public void setActive(boolean active) { isActive = active; }

    @Override
    public String toString() {
        return String.format("User{id=%d, name='%s', email='%s', role='%s', isActive=%s}", 
                           id, name, email, role, isActive);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        User user = (User) o;
        return id == user.id;
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
}

/**
 * Custom exception for user not found
 */
class UserNotFoundException extends Exception {
    public UserNotFoundException(String message) {
        super(message);
    }
}

/**
 * Custom exception for validation errors
 */
class ValidationException extends Exception {
    public ValidationException(String message) {
        super(message);
    }
}

/**
 * User service class for managing users
 */
public class UserService {
    private Map<Integer, User> users;
    private int nextId;

    public UserService() {
        this.users = new HashMap<>();
        this.nextId = 1;
    }

    /**
     * Create a new user
     */
    public User createUser(String name, String email, String role) throws ValidationException {
        validateUserData(name, email);
        
        User user = new User(nextId++, name, email, role);
        users.put(user.getId(), user);
        
        System.out.println("Created user: " + user.getName());
        return user;
    }

    /**
     * Get user by ID
     */
    public User getUserById(int id) throws UserNotFoundException {
        User user = users.get(id);
        if (user == null) {
            throw new UserNotFoundException("User with ID " + id + " not found");
        }
        return user;
    }

    /**
     * Get all users
     */
    public List<User> getAllUsers() {
        return new ArrayList<>(users.values());
    }

    /**
     * Update user
     */
    public User updateUser(int id, String name, String email, String role) 
            throws UserNotFoundException, ValidationException {
        User user = getUserById(id);
        
        if (name != null) {
            user.setName(name);
        }
        if (email != null) {
            validateEmail(email);
            user.setEmail(email);
        }
        if (role != null) {
            user.setRole(role);
        }
        
        System.out.println("Updated user: " + user.getName());
        return user;
    }

    /**
     * Delete user
     */
    public void deleteUser(int id) throws UserNotFoundException {
        User user = getUserById(id);
        users.remove(id);
        System.out.println("Deleted user: " + user.getName());
    }

    /**
     * Search users by name or email
     */
    public List<User> searchUsers(String query) {
        return users.values().stream()
                .filter(user -> user.getName().toLowerCase().contains(query.toLowerCase()) ||
                               user.getEmail().toLowerCase().contains(query.toLowerCase()))
                .collect(Collectors.toList());
    }

    /**
     * Get users by role
     */
    public List<User> getUsersByRole(String role) {
        return users.values().stream()
                .filter(user -> user.getRole().equals(role))
                .collect(Collectors.toList());
    }

    /**
     * Get active users only
     */
    public List<User> getActiveUsers() {
        return users.values().stream()
                .filter(User::isActive)
                .collect(Collectors.toList());
    }

    /**
     * Sort users by name
     */
    public List<User> getUsersSortedByName() {
        return users.values().stream()
                .sorted(Comparator.comparing(User::getName))
                .collect(Collectors.toList());
    }

    /**
     * Validate user data
     */
    private void validateUserData(String name, String email) throws ValidationException {
        if (name == null || name.trim().isEmpty()) {
            throw new ValidationException("Name is required");
        }
        if (email == null || email.trim().isEmpty()) {
            throw new ValidationException("Email is required");
        }
        validateEmail(email);
    }

    /**
     * Validate email format
     */
    private void validateEmail(String email) throws ValidationException {
        String emailRegex = "^[A-Za-z0-9+_.-]+@(.+)$";
        Pattern pattern = Pattern.compile(emailRegex);
        if (!pattern.matcher(email).matches()) {
            throw new ValidationException("Invalid email format");
        }
    }
}

/**
 * Main class demonstrating the user service
 */
public class Main {
    public static void main(String[] args) {
        UserService userService = new UserService();

        try {
            // Create some users
            User user1 = userService.createUser("Alice Johnson", "alice@example.com", "admin");
            User user2 = userService.createUser("Bob Smith", "bob@example.com", "user");
            User user3 = userService.createUser("Charlie Brown", "charlie@example.com", "moderator");

            // Display all users
            System.out.println("\nAll users:");
            List<User> allUsers = userService.getUsersSortedByName();
            for (User user : allUsers) {
                System.out.println("  " + user);
            }

            // Search users
            System.out.println("\nSearch results for 'Alice':");
            List<User> searchResults = userService.searchUsers("Alice");
            for (User user : searchResults) {
                System.out.println("  " + user.getName() + " (" + user.getEmail() + ")");
            }

            // Get users by role
            System.out.println("\nAdmin users:");
            List<User> adminUsers = userService.getUsersByRole("admin");
            for (User user : adminUsers) {
                System.out.println("  " + user.getName() + " (" + user.getEmail() + ")");
            }

            // Update a user
            User updatedUser = userService.updateUser(user2.getId(), null, null, "moderator");
            System.out.println("\nUpdated user: " + updatedUser.getName() + " is now a " + updatedUser.getRole());

            // Get active users
            System.out.println("\nActive users:");
            List<User> activeUsers = userService.getActiveUsers();
            for (User user : activeUsers) {
                System.out.println("  " + user.getName() + " - " + user.getRole());
            }

        } catch (ValidationException | UserNotFoundException e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
}
