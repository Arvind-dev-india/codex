// JavaScript Test Suite - Basic Class
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
        this.isActive = true;
    }

    // Method with parameters
    updateEmail(newEmail) {
        if (this.validateEmail(newEmail)) {
            this.email = newEmail;
            return true;
        }
        return false;
    }

    // Private method (convention)
    validateEmail(email) {
        return email.includes('@') && email.includes('.');
    }

    // Static method
    static createGuest() {
        return new User('Guest', 'guest@example.com');
    }

    // Getter
    get displayName() {
        return `${this.name} (${this.email})`;
    }

    // Setter
    set active(value) {
        this.isActive = Boolean(value);
    }
}

// Function declaration
function processUser(user) {
    console.log(`Processing user: ${user.displayName}`);
    return user.isActive;
}

// Arrow function
const validateUser = (user) => {
    return user && user.email && user.name;
};

// Async function
async function fetchUserData(userId) {
    try {
        const response = await fetch(`/api/users/${userId}`);
        return await response.json();
    } catch (error) {
        console.error('Failed to fetch user:', error);
        throw error;
    }
}

// Export
module.exports = { User, processUser, validateUser, fetchUserData };