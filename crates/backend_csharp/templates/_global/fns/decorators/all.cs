#if NETCOREAPP3_0_OR_GREATER
[MethodImpl(MethodImplOptions.AggressiveOptimization)]
#else
[MethodImpl(MethodImplOptions.AggressiveInlining)]
#endif