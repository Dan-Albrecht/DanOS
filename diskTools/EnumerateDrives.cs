namespace diskTools
{
    using Microsoft.Win32.SafeHandles;
    using System;
    using System.CommandLine.Invocation;
    using System.Runtime.CompilerServices;
    using System.Runtime.InteropServices;
    using Windows.Win32;
    using Windows.Win32.Devices.DeviceAndDriverInstallation;
    using Windows.Win32.Foundation;
    using Windows.Win32.Storage.FileSystem;
    using Windows.Win32.System.Ioctl;

    internal class EnumerateDrives : ICommandHandler
    {
        public static List<(string deviceName, string classInfo)> Fetch()
        {
            var results = new List<(string deviceName, string classInfo)>();

            // https://stackoverflow.com/a/18183115
            SetupDiDestroyDeviceInfoListSafeHandle classDevices = PInvoke.SetupDiGetClassDevs(
                PInvoke.GUID_DEVINTERFACE_DISK,
                null,
                HWND.Null,
                PInvoke.DIGCF_DEVICEINTERFACE | PInvoke.DIGCF_PRESENT);

            if (classDevices.IsInvalid)
            {
                int lastError = Marshal.GetLastWin32Error();
                throw new InvalidOperationException($"Couldn't enumerate USB. Maybe none attached? Error: {lastError}");
            }

            uint deviceIndex = 0;
            SP_DEVICE_INTERFACE_DATA deviceInterface = default;
            deviceInterface.cbSize = (uint)Marshal.SizeOf<SP_DEVICE_INTERFACE_DATA>();

            while (PInvoke.SetupDiEnumDeviceInterfaces(classDevices, null, PInvoke.GUID_DEVINTERFACE_DISK, deviceIndex, ref deviceInterface))
            {
                deviceIndex++;
                string devicePath;

                unsafe
                {
                    uint requiredSize;
                    uint* requiredSizePointer = &requiredSize;

                    _ = PInvoke.SetupDiGetDeviceInterfaceDetail(
                        classDevices,
                        deviceInterface,
                        null,
                        0,
                        requiredSizePointer,
                        null);


                    // BUGBUG: I'm almost certainly doing it wrong with these 'fixed size strings'
                    var structSize = (uint)Marshal.SizeOf<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();

                    fixed (byte* dataBuffer = new byte[requiredSize + structSize])
                    {
                        var deviceInterfaceDetail = (SP_DEVICE_INTERFACE_DETAIL_DATA_W*)dataBuffer;
                        deviceInterfaceDetail->cbSize = structSize;

                        BOOL detailResult = PInvoke.SetupDiGetDeviceInterfaceDetail(
                        classDevices,
                        deviceInterface,
                        deviceInterfaceDetail,
                        requiredSize,
                        null,
                        null);

                        if (detailResult == false)
                        {
                            var winError = (WIN32_ERROR)Marshal.GetLastWin32Error();
                            throw new InvalidOperationException($"Loop ended with unexpected error code: {winError}");
                        }

                        devicePath = MemoryMarshal
                            .CreateReadOnlySpan(ref Unsafe.AsRef(in deviceInterfaceDetail->DevicePath[0]), (int)requiredSize)
                            .SliceAtNull()
                            .ToString();
                    }
                }

                SafeFileHandle disk = PInvoke.CreateFile(
                    devicePath,
                    (uint)GENERIC_ACCESS_RIGHTS.GENERIC_READ,
                    FILE_SHARE_MODE.FILE_SHARE_NONE,
                    null,
                    FILE_CREATION_DISPOSITION.OPEN_EXISTING,
                    FILE_FLAGS_AND_ATTRIBUTES.FILE_ATTRIBUTE_NORMAL,
                    null);

                if (disk.IsInvalid)
                {
                    var winError = (WIN32_ERROR)Marshal.GetLastWin32Error();
                    throw new InvalidOperationException($"CreateFile failed with: {winError}");
                }

                STORAGE_DEVICE_NUMBER deviceNumber = default;
                uint bytesRead;

                unsafe
                {
                    BOOL controlResult = PInvoke.DeviceIoControl(
                        disk,
                        PInvoke.IOCTL_STORAGE_GET_DEVICE_NUMBER,
                        null,
                        0,
                        &deviceNumber,
                        (uint)Marshal.SizeOf<STORAGE_DEVICE_NUMBER>(),
                        &bytesRead,
                        null);

                    if (controlResult == false)
                    {
                        var winError = (WIN32_ERROR)Marshal.GetLastWin32Error();
                        throw new InvalidOperationException($"DeviceIoControl failed with: {winError}");
                    }
                }

                results.Add(($@"\\.\PhysicalDrive{deviceNumber.DeviceNumber}", devicePath));
            }

            results.Sort();
            return results;
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

        private void Run()
        {
            List<(string deviceName, string classInfo)> results = Fetch();
            Display(results);
        }

        public static void Display(List<(string deviceName, string classInfo)> results)
        {
            foreach (var result in results)
            {
                Console.WriteLine($"{result.deviceName}\t{result.classInfo}");
            }
        }
    }
}
