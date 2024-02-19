namespace diskTools
{
    using System;
    using System.Collections.Generic;
    using System.CommandLine.Invocation;
    using System.Linq;
    using System.Text;
    using System.Threading.Tasks;

    internal class Merge
    {
        internal static void DoIt(string bootloaderPath, string diskImagePath, string outputPath)
        {
            if (!File.Exists(bootloaderPath))
            {
                throw new FileNotFoundException($"Bootloader not found: {bootloaderPath}");
            }

            if (!File.Exists(diskImagePath))
            {
                throw new FileNotFoundException($"Disk image not found: {diskImagePath}");
            }

            var loaderBytes = File.ReadAllBytes(bootloaderPath);
            if (loaderBytes.Length != 512)
            {
                throw new InvalidDataException($"Expected to read exactly 512 bytes from loader. Read {loaderBytes.Length}");
            }

            var diskBytes = File.ReadAllBytes(diskImagePath);
            if (diskBytes.Length != 512)
            {
                throw new InvalidDataException($"Expected to read exactly 512 bytes from disk image. Read {diskBytes.Length}");
            }

            for (int i = 440; i < 512; i++)
            {
                loaderBytes[i] = diskBytes[i];
            }

            File.WriteAllBytes(outputPath, loaderBytes);
        }
    }
}
