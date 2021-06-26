using System.IO;
using UnityEditor;
using UnityEngine;

namespace My.Company
{
    class InteroptopusHotReload
    {
        private const string DllSource = @"D:\Development\Source\interoptopus\target\debug\unity_hot_reload.dll";
        private const string DllDst = @"Assets/MyRustLib/Plugins/xxx_sha256_123187.dll";
        private const string InteropDstRoot = @"Assets/MyRustLib/";
        private const string InteropSrcRoot = @"D:\Development\Source\interoptopus\examples\unity_hot_reload\bindings\csharp\";
        private static readonly string[] InteropFiles = {
            @"Interop.cs",
        };

        static void UpdateInteropFiles()
        {
            Debug.Log("Running");
            File.Copy(DllSource, DllDst);

            foreach (var file in InteropFiles)
            {
                
                File.Copy(Path.Combine(InteropSrcRoot, file), Path.Combine(InteropDstRoot, file));
            }
        }
        
        [InitializeOnLoadMethod]
        static void OnProjectLoadedInEditor()
        {
            Debug.Log("Project loaded in Unity Editor 2");
            
            
        }
        
        [MenuItem("Interoptopus/Hot Reload - XXX")]
        static void Init()
        {
            UpdateInteropFiles();
        }
    }
  
}