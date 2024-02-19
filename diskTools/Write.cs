namespace diskTools
{
    using Microsoft.Win32.SafeHandles;
    using System;
    using System.CommandLine.Invocation;
    using System.Runtime.CompilerServices;
    using System.Runtime.InteropServices;
    using System.Threading.Tasks;
    using Windows.Win32;
    using Windows.Win32.Devices.DeviceAndDriverInstallation;
    using Windows.Win32.Devices.Properties;
    using Windows.Win32.Foundation;
    using Windows.Win32.Storage.FileSystem;
    using Windows.Win32.System.Ioctl;

    internal class Write : ICommandHandler
    {
        public int Invoke(InvocationContext context)
        {
            return InvokeAsync(context).ConfigureAwait(false).GetAwaiter().GetResult();
        }

        public async Task<int> InvokeAsync(InvocationContext context)
        {
            await Task.Yield();
            Run();
            return 0;
        }

        private void Run()
        {
            Console.WriteLine("Current drives:");
            var devices = EnumerateDrives.Fetch();
            EnumerateDrives.Display(devices);

            Console.WriteLine();
            Console.WriteLine("Which class do you want to write to and LOSE ALL DATA?");
            string choice = Console.ReadLine() ?? throw new InvalidDataException("How did you manage to submit a null string?!");

            // Force selection by class name as that'll be safer to copy/paste as device numbers can change for many reasons
            string driveSelection = devices.FirstOrDefault(device => device.classInfo == choice).deviceName;

            if (string.IsNullOrEmpty(driveSelection))
            {
                throw new InvalidDataException($"No such class '{choice}'. You must select by class name.");
            }

            if (!choice.StartsWith(@"\\?\usbstor#disk&", StringComparison.Ordinal))
            {
                throw new InvalidDataException($"The choice of '{choice}' doesn't look like a USB disk. This is too risky you might wreck a real disk so you cannot do that. If you know better, update this code.");
            }

            Console.WriteLine();
            Console.WriteLine($"Last chance! Are you really sure you want to use {driveSelection} and LOSE ALL DATA? (y/n)");
            var key = Console.ReadKey(true);

            if(key.Key != ConsoleKey.Y)
            {
                Console.WriteLine("Ok, doing nothing.");
            }

            WriteDrive(driveSelection);
        }

        private void WriteDrive(string driveSelection)
        {
            throw new NotImplementedException();
        }
    }
}
