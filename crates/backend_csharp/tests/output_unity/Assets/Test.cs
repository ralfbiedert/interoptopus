using System;
using System.Collections;
using System.Collections.Generic;
using My.Company;
using Unity.Burst;
using Unity.Collections;
using Unity.Jobs;
using UnityEngine;

// We only use this test to manually check if Interop bindings would compile. 
public class BurstCompileOnlyTestWillNotRun : MonoBehaviour
{

    [BurstCompile(CompileSynchronously = true)]
    private struct MyJob : IJob
    {
        [ReadOnly]
        public NativeArray<uint> uints;

        [ReadOnly]
        public NativeArray<byte> bytes1;

        [ReadOnly]
        public NativeArray<byte> bytes2;
        
        public FunctionPointer<CallbackU8> FUNCTION_POINTER;
        
        [BurstCompile]
        public static byte CallbackU8(byte x0)
        {
            return x0;
        }
        
        public void Execute()
        {
            Interop.ref_mut_simple(out long xx);
            Interop.sleep(10);
            Interop.pattern_ffi_slice_5(bytes1, bytes2);
            Interop.pattern_ffi_slice_6(bytes1, FUNCTION_POINTER.Value);
        }
    }
}

public class CompileOnlyTestWillNotRun : MonoBehaviour
{
 
}
