Pre-compiled .DLLs for plugin tests.

The files in here are semi-automatically produced via a `just` command. Unfortunately 
we can't run this command by default as the `dotnet` build output isn't reproducible.

Another exception is `Newtonsoft.Json.dll`, which we've copied here to simplify testing
plugins with dependencies. 

