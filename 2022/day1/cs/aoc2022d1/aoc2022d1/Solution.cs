using System.Collections.Generic;
using System.IO;

namespace aoc2022d1
{
    public class Solution : ISolution
    {
        public int Solve(Stream data)
        {
            StreamReader reader = new StreamReader(data);
            int answer = 0;
            List<int> cur = new List<int>(10);

            while (reader.ReadLine() is { } line)
            {
                if (line.Length == 0)
                {
                    int total = FoldSum(cur);
                    if (total > answer)
                        answer = total;
                    cur.Clear();
                    continue;
                }

                bool ok = int.TryParse(line, out int parsed);
                if (!ok)
                    return -1;
                cur.Add(parsed);
            }

            return answer;
        }

        private static int FoldSum(IReadOnlyList<int> items)
        {
            int total = 0;
            for (int i = 0; i < items.Count; i++)
                total += items[i];
            return total;
        }
    }
}