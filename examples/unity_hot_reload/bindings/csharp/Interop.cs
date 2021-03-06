// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using My.Company;
#pragma warning restore 0105

namespace My.Company
{
    public static partial class InteropClass
    {
        public const string NativeLib = "unity_hot_reload";

        static InteropClass()
        {
        }


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "do_math")]
        public static extern uint do_math(uint x);

    }



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
