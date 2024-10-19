using System;
using System.IO;
using UnityEditor;
using UnityEngine;

namespace Interoptopus.Utils
{
    // TODO: This needs more love and cleanup.
    // - Make work on any Unity platform (not just Windows)
    // - Copy based on file hash, not random number
    // - Run automatically when file changes
    // - Nicer way to specify paths ...?
    class HotReload
    {
        private const string DllName = @"unity_hot_reload";
        private const string SourceDll = @"../../../target/debug";
        private const string SourceInteropRoot = @"../bindings/csharp";
        private static readonly string[] InteropFiles = {
            @"Interop.cs",
        };
        private const string DestinationAsssetFolder = @"Assets/MyRustLib/";

        
        static void UpdateInteropFiles()
        {
            var random = new System.Random();
            var pluginFolder = Path.Combine(DestinationAsssetFolder, "Plugins");
            
            // Copy plugin
            var targetDllPrefix = $"{DllName}.{Math.Abs(random.Next())}";
            var targetDllFullPath = Path.Combine(pluginFolder, $"{targetDllPrefix}.dll");
            Directory.CreateDirectory(pluginFolder);
            File.Copy(Path.Combine(SourceDll, $"{DllName}.dll"), targetDllFullPath);

            // Copy interop files
            foreach (var file in InteropFiles)
            {
                var sourceFile = Path.Combine(SourceInteropRoot, file); 
                var destFile = Path.Combine(DestinationAsssetFolder, file);

                var text = File.ReadAllText(sourceFile);
                var newText = text.Replace(DllName, targetDllPrefix);
                
                File.Delete(destFile);
                File.WriteAllText(destFile, newText);
            }
            
            Debug.Log("Hot reloading successful.");
        }
        
        [InitializeOnLoadMethod]
        static void OnProjectLoadedInEditor()
        {
            // TODO: Check hash and copy automatically 
            // Debug.Log("Project loaded in Unity Editor 2");
        }
        
        [MenuItem("Interoptopus/Hot Reload")]
        static void Init()
        {
            UpdateInteropFiles();
        }
    }
  
}