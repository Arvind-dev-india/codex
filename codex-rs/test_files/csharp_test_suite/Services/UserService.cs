using System;
using System.Collections.Generic;
using System.Linq;
using TestApp.Models;
using TestApp.Data;

namespace TestApp.Services
{
    public class UserService : IUserService
    {
        private readonly IRepository<User> _userRepository;
        private readonly IOrderService _orderService;

        public UserService(IRepository<User> userRepository, IOrderService orderService)
        {
            _userRepository = userRepository ?? throw new ArgumentNullException(nameof(userRepository));
            _orderService = orderService ?? throw new ArgumentNullException(nameof(orderService));
        }

        public User CreateUser(string name, string email)
        {
            if (string.IsNullOrWhiteSpace(name))
                throw new ArgumentException("Name cannot be empty", nameof(name));
            
            if (string.IsNullOrWhiteSpace(email))
                throw new ArgumentException("Email cannot be empty", nameof(email));

            if (IsEmailExists(email))
                throw new InvalidOperationException($"User with email {email} already exists");

            var user = new User
            {
                Id = GenerateUserId(),
                Name = name,
                Email = email
            };

            _userRepository.Add(user);
            LogUserCreation(user);
            return user;
        }

        public User GetUser(int id)
        {
            var user = _userRepository.GetById(id);
            if (user != null)
            {
                LoadUserOrders(user);
            }
            return user;
        }

        public List<User> GetAllUsers()
        {
            var users = _userRepository.GetAll();
            foreach (var user in users)
            {
                LoadUserOrders(user);
            }
            return users;
        }

        public bool UpdateUser(User user)
        {
            if (user == null)
                throw new ArgumentNullException(nameof(user));

            var existingUser = _userRepository.GetById(user.Id);
            if (existingUser == null)
                return false;

            ValidateUserUpdate(user, existingUser);
            _userRepository.Update(user);
            LogUserUpdate(user);
            return true;
        }

        public bool DeleteUser(int id)
        {
            var user = _userRepository.GetById(id);
            if (user == null)
                return false;

            if (HasActiveOrders(user))
                throw new InvalidOperationException("Cannot delete user with active orders");

            _userRepository.Delete(id);
            LogUserDeletion(user);
            return true;
        }

        public List<User> SearchUsers(string searchTerm)
        {
            if (string.IsNullOrWhiteSpace(searchTerm))
                return new List<User>();

            var allUsers = GetAllUsers();
            return allUsers.Where(u => 
                u.Name.Contains(searchTerm, StringComparison.OrdinalIgnoreCase) ||
                u.Email.Contains(searchTerm, StringComparison.OrdinalIgnoreCase)
            ).ToList();
        }

        private bool IsEmailExists(string email)
        {
            var users = _userRepository.GetAll();
            return users.Any(u => u.Email.Equals(email, StringComparison.OrdinalIgnoreCase));
        }

        private int GenerateUserId()
        {
            var users = _userRepository.GetAll();
            return users.Count > 0 ? users.Max(u => u.Id) + 1 : 1;
        }

        private void LoadUserOrders(User user)
        {
            var orders = _orderService.GetUserOrders(user.Id);
            user.Orders = orders;
        }

        private void ValidateUserUpdate(User newUser, User existingUser)
        {
            if (newUser.Email != existingUser.Email && IsEmailExists(newUser.Email))
            {
                throw new InvalidOperationException($"Email {newUser.Email} is already in use");
            }
        }

        private bool HasActiveOrders(User user)
        {
            var orders = _orderService.GetUserOrders(user.Id);
            return orders.Any(o => o.Status != OrderStatus.Delivered && o.Status != OrderStatus.Cancelled);
        }

        private void LogUserCreation(User user)
        {
            Console.WriteLine($"User created: {user.Name} ({user.Email})");
        }

        private void LogUserUpdate(User user)
        {
            Console.WriteLine($"User updated: {user.Name} ({user.Email})");
        }

        private void LogUserDeletion(User user)
        {
            Console.WriteLine($"User deleted: {user.Name} ({user.Email})");
        }
    }
}