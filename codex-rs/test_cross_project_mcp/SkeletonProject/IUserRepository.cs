using SkeletonProject.Models;

namespace SkeletonProject.Services
{
    public interface IUserRepository
    {
        User CreateUser(User user);
        User GetUserById(int id);
    }
}