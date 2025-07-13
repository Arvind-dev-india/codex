// TypeScript Test Suite - Basic Class
interface IUser {
    name: string;
    email: string;
    isActive: boolean;
    updateEmail(newEmail: string): boolean;
}

interface UserConfig {
    readonly id: number;
    name: string;
    email?: string;
    permissions: string[];
}

// Generic interface
interface Repository<T> {
    findById(id: number): Promise<T | null>;
    save(entity: T): Promise<T>;
    delete(id: number): Promise<boolean>;
}

// Abstract class
abstract class BaseEntity {
    protected readonly id: number;
    protected createdAt: Date;

    constructor(id: number) {
        this.id = id;
        this.createdAt = new Date();
    }

    abstract validate(): boolean;
    
    getId(): number {
        return this.id;
    }
}

// Class implementing interface and extending abstract class
class User extends BaseEntity implements IUser {
    public name: string;
    public email: string;
    public isActive: boolean;
    private _permissions: Set<string>;

    constructor(id: number, name: string, email: string) {
        super(id);
        this.name = name;
        this.email = email;
        this.isActive = true;
        this._permissions = new Set();
    }

    // Method implementation
    updateEmail(newEmail: string): boolean {
        if (this.validateEmail(newEmail)) {
            this.email = newEmail;
            return true;
        }
        return false;
    }

    // Private method
    private validateEmail(email: string): boolean {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }

    // Override abstract method
    validate(): boolean {
        return this.name.length > 0 && this.validateEmail(this.email);
    }

    // Static method with generic
    static create<T extends User>(
        UserClass: new (id: number, name: string, email: string) => T,
        config: UserConfig
    ): T {
        return new UserClass(config.id, config.name, config.email || '');
    }

    // Getter with type annotation
    get permissions(): string[] {
        return Array.from(this._permissions);
    }

    // Method with union types
    setStatus(status: 'active' | 'inactive' | 'pending'): void {
        this.isActive = status === 'active';
    }
}

// Generic function
function processEntity<T extends BaseEntity>(entity: T): T {
    if (entity.validate()) {
        console.log(`Processing entity with ID: ${entity.getId()}`);
        return entity;
    }
    throw new Error('Invalid entity');
}

// Type alias
type UserAction = 'create' | 'update' | 'delete' | 'view';

// Enum
enum UserRole {
    ADMIN = 'admin',
    USER = 'user',
    GUEST = 'guest',
    MODERATOR = 'moderator'
}

// Namespace
namespace UserUtils {
    export function formatName(user: IUser): string {
        return user.name.toUpperCase();
    }

    export function isValidRole(role: string): role is UserRole {
        return Object.values(UserRole).includes(role as UserRole);
    }
}

// Export
export { User, IUser, UserConfig, Repository, UserAction, UserRole, UserUtils };