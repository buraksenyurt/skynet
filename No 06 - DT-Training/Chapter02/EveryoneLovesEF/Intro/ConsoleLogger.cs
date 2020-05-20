using Microsoft.Extensions.Logging;
using System;

namespace Intro
{
    public class ConsoleLoggerProvider
        : ILoggerProvider
    {
        public ILogger CreateLogger(string categoryName)
        {
            return new ConsoleLogger();
        }

        public void Dispose() { }
    }

    public class ConsoleLogger : ILogger
    {
        public IDisposable BeginScope<T>(T state) => null;
        public bool IsEnabled(LogLevel level)
        {
            switch (level)
            {
                case LogLevel.Trace:
                case LogLevel.Information:
                case LogLevel.None:
                    return false;
                case LogLevel.Debug:
                case LogLevel.Warning:
                case LogLevel.Error:
                case LogLevel.Critical:
                default:
                    return true;
            };
        }
        public void Log<T>(LogLevel level, EventId eventId, T state, Exception exception, Func<T, Exception, string> formatter)
        {
            Console.Write($"Level: {level}, Event ID:{eventId.Id}");
            if (state != null)
            {
                Console.Write($", State:{state}");
            }
            if (exception != null)
            {
                Console.Write($", Exception: {exception.Message}");
            }
            Console.WriteLine();
        }
    }
}