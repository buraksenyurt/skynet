using System;
using System.Linq;
using System.Collections.Generic;
using System.IO;

namespace AdventureWorks.Common
{
    public class Utility
    {
        public int Sum(int a1, int a2, int a3, int a4)
        {
            return a1 + a2 + a3 + a4;
        }

        public int Sum(int a1, int a2)
        {
            return a1 + a2;
        }

        public double FindGreater(double x, double y)
        {
            throw new NotImplementedException();
        }

        public void SendEmail(string to, string from, string subject, string body, string type, string cc)
        {
            // Mail gönderme işlemi

            var trimmedTo = to.TrimEnd(' ');
            var trimmedFrom = from.TrimEnd(' ');
            if (subject.Length > 25)
            {
                throw new OverflowException("Konu başlığı karakter sayısı çok fazla");
            }

            var mailDate = DateTime.Today;
        }
    }

    public class Helper
    {
        public static string CombineMails(string mail1, string mail2, string mail3, string mail4, string mail5)
        {
            return $"{mail1};{mail2};{mail3};{mail4};{mail5}";
        }
    }
}
