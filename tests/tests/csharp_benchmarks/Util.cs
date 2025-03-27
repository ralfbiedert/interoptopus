
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

namespace Interoptopus
{
    public delegate void Run();

    public class MeasureResult
    {
        private static long _calibrationTicks = 0;

        long _n;
        long _totalTicks;

        public double MicroPer1000()
        {
            var n = (double) _n;
            var ticks_for_all_n = (double) _totalTicks;

            // 1 tick = 100 nanos
            var micros_for_all_n = ticks_for_all_n / 10;
            var micros_for_one_n = micros_for_all_n / n;
            var micros_for_1000_n = 1000 * micros_for_one_n;

            // return (100 * ticks_for_all_n / n);
            return micros_for_1000_n;
        }

        public MeasureResult(long n, long totalTicks)
        {
            _n = n;
            _totalTicks = totalTicks;
        }

        public static void Calibrate(uint n, Run r)
        {
            var result = Measure(n, r);
            _calibrationTicks = result._totalTicks;
        }

        public static MeasureResult Measure(uint n, Run r)
        {

            for (var i = 0; i < n; i++)
            {
                r.Invoke();
            }

            var stopwatch = new Stopwatch();
            stopwatch.Start();
            for (var i = 0; i < n; i++)
            {
                r.Invoke();
            }
            stopwatch.Stop();

            return new MeasureResult( n, stopwatch.ElapsedTicks - _calibrationTicks);
        }

    }

    class Entry
    {
        public string Name;
        public MeasureResult Result;
    }

    public class MarkdownTableWriter
    {
        private List<Entry> Entries = new List<Entry>();

        public void Add(string name, MeasureResult result)
        {
            Console.WriteLine($"{name}: {result.MicroPer1000():F0}");
            Entries.Add(new Entry()
            {
                Name = name,
                Result = result
            });
        }

        public void Write(string file)
        {
            var header = @"
# FFI Call Overheads

The numbers below are to help FFI design decisions by giving order-of-magnitude estimates how
expensive certain constructs are.

## Notes

- Times were determined by running the given construct 100k times, taking the elapsed time in ticks,
and computing the cost per 1k invocations.

- The time of the called function is included.

- However, the reference project was written so that each function is _minimal_, i.e., any similar
function you wrote, would have to at least as expensive operations if it were to do anything sensible with
the given type.

- The list is ad-hoc, PRs adding more tests to `Benchmark.cs` are welcome.

- Bindings were generated with the C# `use_unsafe` config, which dramatically (between 2x and 150x(!)) speeds
  up slice access and copies in .NET and Unity, [see the FAQ for details](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#existing-backends).

## System

The following system was used:

```
System: AMD Ryzen 9 7950X3D, 64 GB RAM; Windows 11
rustc: stable (i.e., 1.85 or later)
profile: --release
.NET: v9.0
```

## Results

| Construct | ns per call |
| --- | --- |
";

            using StreamWriter sw = File.CreateText(file);
            sw.Write(header);
            foreach (var entry in Entries)
            {
                sw.WriteLine($"| `{entry.Name}` | {(long) entry.Result.MicroPer1000():F0} |");
            }
        }
    }
}
