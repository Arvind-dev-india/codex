using System;
using SkeletonProject.Models;

namespace MainProject.Models
{
    /// <summary>
    /// Main project User wrapper that extends the SkeletonProject User
    /// This demonstrates the same-named symbol issue where both projects have a "User" class
    /// </summary>
    public class User
    {
        private readonly SkeletonProject.Models.User _baseUser;
        
        public User()
        {
            _baseUser = new SkeletonProject.Models.User();
        }
        
        public User(SkeletonProject.Models.User baseUser)
        {
            _baseUser = baseUser ?? throw new ArgumentNullException(nameof(baseUser));
        }
        
        public int Id 
        { 
            get => _baseUser.Id; 
            set => _baseUser.Id = value; 
        }
        
        public string Name 
        { 
            get => _baseUser.Name; 
            set => _baseUser.Name = value; 
        }
        
        public string Email 
        { 
            get => _baseUser.Email; 
            set => _baseUser.Email = value; 
        }
        
        // Additional functionality in the main project wrapper
        public string DisplayName => $"{Name} ({Email})";
        
        public void Validate()
        {
            _baseUser.ValidateUser();
        }
        
        // Wrapper method that uses the cross-project functionality
        public SkeletonProject.Models.User GetBaseUser()
        {
            return _baseUser;
        }
    }
}