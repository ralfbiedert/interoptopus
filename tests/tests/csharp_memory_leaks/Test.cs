using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using Xunit;

public class TestMain
{
    // We only have a single main entry point so tests don't run in parallel
    [Fact]
    public void does_not_leak_memory()
    {
        var N = 10_000;
        var MAX_DELTA_KB = 16 * 1024;
        var beforeMemory = 0l;
        var midMemory = 0l;
        var afterMemory = 0l;

        for (var i = 0; i < N; ++i)
        {
            if (i == 10) beforeMemory = MemoryUsage();
            if (i == N / 2) midMemory = MemoryUsage();
            if (i == N - 1) afterMemory = MemoryUsage();

            TestPrimitives.Check();
            TestArrays.Check();
            TestRefs.Check();
            TestPatternCallbacks.Check();
            TestPatternServiceAsync.Check().Wait();
        }

        var first_half = Math.Abs(midMemory - beforeMemory);
        var second_half = Math.Abs(afterMemory - midMemory);
        // Assert.True(first_half < (MAX_DELTA_KB * 1024), $"Memory leak detected: {beforeMemory} -> {midMemory} -> {afterMemory}.");
        Assert.True(second_half < (MAX_DELTA_KB * 1024), $"Memory leak detected: {beforeMemory} -> {midMemory} -> {afterMemory}.");
    }

    static long MemoryUsage()
    {
        // System.Diagnostics.Process.GetProcesses().
        // return GC.GetTotalMemory(true);
        // GC.Collect();
        // return GC.GetAllocatedBytesForCurrentThread();

        var proc = Process.GetCurrentProcess();
        if (GetProcessMemoryInfo(proc.Handle, out PROCESS_MEMORY_COUNTERS counters, (uint)Marshal.SizeOf(typeof(PROCESS_MEMORY_COUNTERS))))
        {
            return (long)counters.WorkingSetSize;
        }
        throw new Exception("GetProcessMemoryInfo failed");
    }

    [DllImport("psapi.dll", SetLastError = true)]
    static extern bool GetProcessMemoryInfo(IntPtr hProcess, out PROCESS_MEMORY_COUNTERS counters, uint size);

    [StructLayout(LayoutKind.Sequential)]
    public struct PROCESS_MEMORY_COUNTERS
    {
        public uint cb;
        public uint PageFaultCount;
        public ulong PeakWorkingSetSize;
        public ulong WorkingSetSize;
        public ulong QuotaPeakPagedPoolUsage;
        public ulong QuotaPagedPoolUsage;
        public ulong QuotaPeakNonPagedPoolUsage;
        public ulong QuotaNonPagedPoolUsage;
        public ulong PagefileUsage;
        public ulong PeakPagefileUsage;
    }
}
