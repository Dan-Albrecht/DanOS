using Windows.Win32;
using Windows.Win32.Foundation;
using Windows.Win32.Storage.FileSystem;
using Microsoft.Win32.SafeHandles;
using System.Runtime.InteropServices;
using System.CommandLine;
using diskTools;

var root = new RootCommand("Various commands to operator on a disk at the raw level");
var dump = new Command("dump", "Dump the hardcoded disk to stdout (you should redirect to a file).")
{
    Handler = new Dump(),
};
root.Add(dump);
await root.InvokeAsync(args);



/*


*/