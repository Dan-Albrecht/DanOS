namespace diskTools
{
    using Microsoft.Win32.SafeHandles;
    using System;
    using System.CommandLine.Invocation;
    using System.Runtime.InteropServices;
    using System.Threading.Tasks;
    using Windows.Win32;
    using Windows.Win32.Foundation;
    using Windows.Win32.Storage.FileSystem;

    internal class Dump : ICommandHandler
    {
        const string path = @"\\.\PhysicalDrive4";

        public static void Run()
        {
            SafeFileHandle usbDrive = PInvoke.CreateFile(
                path,
                (uint)GENERIC_ACCESS_RIGHTS.GENERIC_READ,
                FILE_SHARE_MODE.FILE_SHARE_NONE,
                null,
                FILE_CREATION_DISPOSITION.OPEN_EXISTING,
                FILE_FLAGS_AND_ATTRIBUTES.FILE_ATTRIBUTE_NORMAL,
                null);

            if (usbDrive.IsInvalid)
            {
                int lastError = Marshal.GetLastWin32Error();
                throw new InvalidDataException($"Couldn't open the drive: {lastError}");
            }

            const int bytesDesired = 512;
            byte[] buffer = new byte[bytesDesired];
            Span<byte> bufferSpan = buffer;
            uint bytesRead = 0;

            unsafe
            {

                BOOL ret = PInvoke.ReadFile(
                    usbDrive,
                    bufferSpan,
                    &bytesRead,
                    null);

                if (ret == false)
                {
                    throw new InvalidDataException("Read failed");
                }

            }

            if (bytesRead != bytesDesired)
            {
                throw new InvalidDataException($"Expected to read {bytesDesired}, but read {bytesRead}");
            }

            Stream stdOut = Console.OpenStandardOutput();
            stdOut.Write(bufferSpan);
            stdOut.Flush();
        }

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
    }
}
