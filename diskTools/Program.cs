﻿using diskTools;
using System.CommandLine;

var mergeCommand = new Command("merge", "Merge a bootloader into a disk image. Reads bytes 0..440 from bootloader and 440..512 from disk image.");

var bootloaderPath = new Argument<string>(
    name: "bootloaderPath",
    description: "Path to the bootloader to write."
    );

var diskImagePath = new Argument<string>(
    name: "diskImagePath",
    description: "Path to the disk image."
    );

var outputPath = new Argument<string>(
    name: "outputPath",
    description: "Path to the write output. Will be overwritten if exists."
    );

var classPath = new Argument<string>(
    name: "deviceClassName",
    description: "Full device class path to write to. This will be overwritten!!"
    );

mergeCommand.AddArgument(bootloaderPath);
mergeCommand.AddArgument(diskImagePath);
mergeCommand.AddArgument(outputPath);
mergeCommand.SetHandler(Merge.DoIt, bootloaderPath, diskImagePath, outputPath);

var dangerousWrite = new Command("dangousWrite", "Non-interactive write to a disk. This is double-DANGEROUS.");
dangerousWrite.AddArgument(diskImagePath);
dangerousWrite.AddArgument(classPath);
dangerousWrite.SetHandler(Write.Dangerous, diskImagePath, classPath);

var root = new RootCommand("Various commands to operator on a disk at the raw level")
{
    new Command("dump", "Dump the hardcoded disk to stdout (you should redirect to a file).")
    {
        Handler = new Dump(),
    },
    new Command("list", "List potential drives we can write to.")
    {
        Handler = new EnumerateDrives(),
    },
    new Command("write", "Write to a disk. Must be run interactive, this is DANGEROUS.")
    {
        Handler = new Write(),
    },
    mergeCommand,
    dangerousWrite,
};

return await root.InvokeAsync(args);
