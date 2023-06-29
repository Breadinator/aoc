using System;
using System.IO;

namespace aoc2022d1
{
    public static class Program
    {
        private static ISolution SolutionFactory() => new Solution();
        
        public static int Main(string[] args)
        {
            if (args.Length == 0)
            {
                Console.WriteLine("Expected file path for input data as an argument.");
                return 1;
            }

            int answer;
            ISolution solution = SolutionFactory();
            FileStream fs = File.OpenRead(args[0]);
            
            try
            {
                answer = solution.Solve(fs);
            }
            finally
            {
                fs.Close();
            }
            
            Console.WriteLine($"The answer is: {answer}"); // why does this box lmao i hate c#
            return 0;
        }
    }
}