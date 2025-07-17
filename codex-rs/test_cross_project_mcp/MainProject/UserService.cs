using System;
using System.Collections.Generic;
using SkeletonProject.Models;
using SkeletonProject.Services;
using SkeletonProject.Utils;

namespace MainProject.Services
{
    public class UserService : IUserRepository
    {
        private readonly List<User> _users = new List<User>();
        
        public User CreateUser(User user)
        {
            if (!ValidationHelper.IsValidEmail(user.Email))
                throw new ArgumentException("Invalid email");
            
            user.ValidateUser();
            _users.Add(user);
            return user;
        }
        
        public User GetUserById(int id)
        {
            return _users.Find(u => u.Id == id);
        }
    }
}