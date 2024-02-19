using diskTools;
using System.CommandLine;

var root = new RootCommand("Various commands to operator on a disk at the raw level")
{
    new Command("dump", "Dump the hardcoded disk to stdout (you should redirect to a file).")
    {
        Handler = new Dump(),
    },
    new Command("list", "List potential drives we can write to.")
    {
        Handler = new Dump(),
    },
    new Command("write", "Write to a disk. Must be run interactive, this is DANGEROUS.")
    {
        Handler = new Write(),
    }
};

await root.InvokeAsync(args);
