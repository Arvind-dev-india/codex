using System.Text.RegularExpressions;

namespace SkeletonProject.Utils
{
    public static class ValidationHelper
    {
        public static bool IsValidEmail(string email)
        {
            return !string.IsNullOrEmpty(email) && email.Contains("@");
        }
    }
}