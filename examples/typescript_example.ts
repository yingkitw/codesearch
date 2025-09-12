// TypeScript example demonstrating various patterns for code search

interface User {
    id: number;
    name: string;
    email: string;
    role: 'admin' | 'user' | 'moderator';
    createdAt: Date;
    isActive: boolean;
}

interface CreateUserRequest {
    name: string;
    email: string;
    role?: User['role'];
}

interface UpdateUserRequest {
    name?: string;
    email?: string;
    role?: User['role'];
    isActive?: boolean;
}

// Generic API response type
interface ApiResponse<T> {
    success: boolean;
    data?: T;
    error?: string;
    message?: string;
}

// Custom error class
class UserNotFoundError extends Error {
    constructor(userId: number) {
        super(`User with ID ${userId} not found`);
        this.name = 'UserNotFoundError';
    }
}

class ValidationError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'ValidationError';
    }
}

// User service class
class UserService {
    private users: Map<number, User> = new Map();
    private nextId: number = 1;

    // Create a new user
    async createUser(userData: CreateUserRequest): Promise<ApiResponse<User>> {
        try {
            this.validateUserData(userData);
            
            const user: User = {
                id: this.nextId++,
                name: userData.name,
                email: userData.email,
                role: userData.role || 'user',
                createdAt: new Date(),
                isActive: true
            };

            this.users.set(user.id, user);
            
            return {
                success: true,
                data: user,
                message: 'User created successfully'
            };
        } catch (error) {
            return {
                success: false,
                error: error instanceof Error ? error.message : 'Unknown error'
            };
        }
    }

    // Get user by ID
    async getUserById(id: number): Promise<ApiResponse<User>> {
        const user = this.users.get(id);
        if (!user) {
            throw new UserNotFoundError(id);
        }
        
        return {
            success: true,
            data: user
        };
    }

    // Update user
    async updateUser(id: number, updates: UpdateUserRequest): Promise<ApiResponse<User>> {
        try {
            const user = this.users.get(id);
            if (!user) {
                throw new UserNotFoundError(id);
            }

            if (updates.email) {
                this.validateEmail(updates.email);
            }

            const updatedUser = { ...user, ...updates };
            this.users.set(id, updatedUser);

            return {
                success: true,
                data: updatedUser,
                message: 'User updated successfully'
            };
        } catch (error) {
            return {
                success: false,
                error: error instanceof Error ? error.message : 'Unknown error'
            };
        }
    }

    // Delete user
    async deleteUser(id: number): Promise<ApiResponse<void>> {
        const user = this.users.get(id);
        if (!user) {
            throw new UserNotFoundError(id);
        }

        this.users.delete(id);
        
        return {
            success: true,
            message: 'User deleted successfully'
        };
    }

    // Get all users
    async getAllUsers(): Promise<ApiResponse<User[]>> {
        const users = Array.from(this.users.values());
        return {
            success: true,
            data: users
        };
    }

    // Search users
    async searchUsers(query: string): Promise<ApiResponse<User[]>> {
        const users = Array.from(this.users.values());
        const filteredUsers = users.filter(user => 
            user.name.toLowerCase().includes(query.toLowerCase()) ||
            user.email.toLowerCase().includes(query.toLowerCase())
        );

        return {
            success: true,
            data: filteredUsers
        };
    }

    // Private validation methods
    private validateUserData(userData: CreateUserRequest): void {
        if (!userData.name || userData.name.trim().length === 0) {
            throw new ValidationError('Name is required');
        }
        
        if (!userData.email) {
            throw new ValidationError('Email is required');
        }
        
        this.validateEmail(userData.email);
    }

    private validateEmail(email: string): void {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email)) {
            throw new ValidationError('Invalid email format');
        }
    }
}

// Utility functions
function formatUserDisplay(user: User): string {
    return `${user.name} (${user.email}) - ${user.role}`;
}

function isAdminUser(user: User): boolean {
    return user.role === 'admin';
}

// Async function to demonstrate error handling
async function handleUserOperation<T>(
    operation: () => Promise<ApiResponse<T>>
): Promise<T> {
    try {
        const result = await operation();
        if (!result.success) {
            throw new Error(result.error || 'Operation failed');
        }
        return result.data!;
    } catch (error) {
        console.error('Operation failed:', error);
        throw error;
    }
}

// Main execution
async function main() {
    const userService = new UserService();

    try {
        // Create users
        const user1 = await handleUserOperation(() => 
            userService.createUser({
                name: 'Alice Johnson',
                email: 'alice@example.com',
                role: 'admin'
            })
        );

        const user2 = await handleUserOperation(() => 
            userService.createUser({
                name: 'Bob Smith',
                email: 'bob@example.com',
                role: 'user'
            })
        );

        console.log('Created users:', [user1, user2].map(formatUserDisplay));

        // Search users
        const searchResults = await userService.searchUsers('Alice');
        console.log('Search results:', searchResults.data?.map(formatUserDisplay));

        // Update user
        const updatedUser = await handleUserOperation(() => 
            userService.updateUser(user2.id, { role: 'moderator' })
        );
        console.log('Updated user:', formatUserDisplay(updatedUser));

    } catch (error) {
        console.error('Error in main:', error);
    }
}

// Export for module usage
export { UserService, User, CreateUserRequest, UpdateUserRequest, ApiResponse };

// Run main if this is the entry point
if (require.main === module) {
    main();
}
