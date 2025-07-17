using System;

namespace SkeletonProject.Models
{
    public class User
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Email { get; set; }
        
        public virtual void ValidateUser()
        {
            if (string.IsNullOrEmpty(Name))
                throw new ArgumentException("Name is required");
        }
    }
}