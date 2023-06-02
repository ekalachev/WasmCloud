using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            var name = Environment.GetEnvironmentVariable("NAME") ?? "World";
            Console.WriteLine($"Hello, {name}!");
        }
    }
}
