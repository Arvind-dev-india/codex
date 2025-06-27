using System;
using System.Collections.Generic;
using TestApp.Models;
using TestApp.Services;

namespace TestApp.Controllers
{
    public class UserController
    {
        private readonly IUserService _userService;
        private readonly IOrderService _orderService;

        public UserController(IUserService userService, IOrderService orderService)
        {
            _userService = userService ?? throw new ArgumentNullException(nameof(userService));
            _orderService = orderService ?? throw new ArgumentNullException(nameof(orderService));
        }

        public ApiResponse<User> CreateUser(CreateUserRequest request)
        {
            try
            {
                ValidateCreateUserRequest(request);
                var user = _userService.CreateUser(request.Name, request.Email);
                return CreateSuccessResponse(user, "User created successfully");
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<User>(ex.Message);
            }
        }

        public ApiResponse<User> GetUser(int id)
        {
            try
            {
                var user = _userService.GetUser(id);
                if (user == null)
                {
                    return CreateNotFoundResponse<User>("User not found");
                }
                return CreateSuccessResponse(user);
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<User>(ex.Message);
            }
        }

        public ApiResponse<List<User>> GetAllUsers()
        {
            try
            {
                var users = _userService.GetAllUsers();
                return CreateSuccessResponse(users);
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<List<User>>(ex.Message);
            }
        }

        public ApiResponse<User> UpdateUser(int id, UpdateUserRequest request)
        {
            try
            {
                var existingUser = _userService.GetUser(id);
                if (existingUser == null)
                {
                    return CreateNotFoundResponse<User>("User not found");
                }

                UpdateUserFromRequest(existingUser, request);
                var success = _userService.UpdateUser(existingUser);
                
                if (success)
                {
                    return CreateSuccessResponse(existingUser, "User updated successfully");
                }
                else
                {
                    return CreateErrorResponse<User>("Failed to update user");
                }
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<User>(ex.Message);
            }
        }

        public ApiResponse<bool> DeleteUser(int id)
        {
            try
            {
                var success = _userService.DeleteUser(id);
                if (success)
                {
                    return CreateSuccessResponse(true, "User deleted successfully");
                }
                else
                {
                    return CreateNotFoundResponse<bool>("User not found");
                }
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<bool>(ex.Message);
            }
        }

        public ApiResponse<List<User>> SearchUsers(string searchTerm)
        {
            try
            {
                var users = _userService.SearchUsers(searchTerm);
                return CreateSuccessResponse(users);
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<List<User>>(ex.Message);
            }
        }

        public ApiResponse<List<Order>> GetUserOrders(int userId)
        {
            try
            {
                var user = _userService.GetUser(userId);
                if (user == null)
                {
                    return CreateErrorResponse<List<Order>>("User not found");
                }

                var orders = _orderService.GetUserOrders(userId);
                return CreateSuccessResponse(orders);
            }
            catch (Exception ex)
            {
                return CreateErrorResponse<List<Order>>(ex.Message);
            }
        }

        private void ValidateCreateUserRequest(CreateUserRequest request)
        {
            if (request == null)
                throw new ArgumentNullException(nameof(request));
            
            if (string.IsNullOrWhiteSpace(request.Name))
                throw new ArgumentException("Name is required");
            
            if (string.IsNullOrWhiteSpace(request.Email))
                throw new ArgumentException("Email is required");
            
            if (!IsValidEmail(request.Email))
                throw new ArgumentException("Invalid email format");
        }

        private void UpdateUserFromRequest(User user, UpdateUserRequest request)
        {
            if (!string.IsNullOrWhiteSpace(request.Name))
                user.Name = request.Name;
            
            if (!string.IsNullOrWhiteSpace(request.Email))
                user.Email = request.Email;
        }

        private bool IsValidEmail(string email)
        {
            return email.Contains("@") && email.Contains(".");
        }

        private ApiResponse<T> CreateSuccessResponse<T>(T data, string message = null)
        {
            return new ApiResponse<T>
            {
                Success = true,
                Data = data,
                Message = message
            };
        }

        private ApiResponse<T> CreateErrorResponse<T>(string message)
        {
            return new ApiResponse<T>
            {
                Success = false,
                Message = message
            };
        }

        private ApiResponse<T> CreateNotFoundResponse<T>(string message)
        {
            return new ApiResponse<T>
            {
                Success = false,
                Message = message
            };
        }
    }

    public class CreateUserRequest
    {
        public string Name { get; set; }
        public string Email { get; set; }
    }

    public class UpdateUserRequest
    {
        public string Name { get; set; }
        public string Email { get; set; }
    }

    public class ApiResponse<T>
    {
        public bool Success { get; set; }
        public T Data { get; set; }
        public string Message { get; set; }
    }
}