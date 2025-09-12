// JavaScript example demonstrating various patterns for code search

class UserManager {
    constructor() {
        this.users = new Map();
        this.nextId = 1;
    }

    // Add a new user
    addUser(userData) {
        const user = {
            id: this.nextId++,
            name: userData.name,
            email: userData.email,
            role: userData.role || 'user',
            createdAt: new Date(),
            isActive: true
        };
        
        this.users.set(user.id, user);
        console.log(`User added: ${user.name}`);
        return user;
    }

    // Get user by ID
    getUserById(id) {
        return this.users.get(id);
    }

    // Get all users
    getAllUsers() {
        return Array.from(this.users.values());
    }

    // Update user
    updateUser(id, updates) {
        const user = this.users.get(id);
        if (user) {
            Object.assign(user, updates);
            console.log(`User updated: ${user.name}`);
            return user;
        }
        return null;
    }

    // Delete user
    deleteUser(id) {
        const user = this.users.get(id);
        if (user) {
            this.users.delete(id);
            console.log(`User deleted: ${user.name}`);
            return true;
        }
        return false;
    }

    // Search users by name
    searchUsersByName(name) {
        const results = [];
        for (const user of this.users.values()) {
            if (user.name.toLowerCase().includes(name.toLowerCase())) {
                results.push(user);
            }
        }
        return results;
    }

    // Get users by role
    getUsersByRole(role) {
        return Array.from(this.users.values()).filter(user => user.role === role);
    }
}

// Utility functions
function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

function formatUserInfo(user) {
    return `${user.name} (${user.email}) - ${user.role}`;
}

// Async function example
async function fetchUserData(userId) {
    try {
        const response = await fetch(`/api/users/${userId}`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const userData = await response.json();
        return userData;
    } catch (error) {
        console.error('Error fetching user data:', error);
        throw error;
    }
}

// Event handling
function setupEventListeners() {
    document.addEventListener('DOMContentLoaded', function() {
        const userForm = document.getElementById('userForm');
        if (userForm) {
            userForm.addEventListener('submit', handleUserSubmit);
        }
    });
}

function handleUserSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    const userData = {
        name: formData.get('name'),
        email: formData.get('email'),
        role: formData.get('role')
    };
    
    if (validateEmail(userData.email)) {
        userManager.addUser(userData);
        event.target.reset();
    } else {
        alert('Please enter a valid email address');
    }
}

// Main execution
const userManager = new UserManager();

// Add some sample users
userManager.addUser({
    name: 'John Doe',
    email: 'john@example.com',
    role: 'admin'
});

userManager.addUser({
    name: 'Jane Smith',
    email: 'jane@example.com',
    role: 'user'
});

userManager.addUser({
    name: 'Bob Johnson',
    email: 'bob@example.com',
    role: 'moderator'
});

// Demonstrate functionality
console.log('All users:', userManager.getAllUsers());
console.log('Admin users:', userManager.getUsersByRole('admin'));
console.log('Search results for "John":', userManager.searchUsersByName('John'));

// Setup event listeners when DOM is ready
if (typeof document !== 'undefined') {
    setupEventListeners();
}
