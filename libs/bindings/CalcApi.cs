namespace Scaffold.Bindings;

public static unsafe partial class CalcNativeGen
{
    public const int ERR_INVALID_HANDLE = -1;
    public const int ERR_BAD_ARGUMENT = -2;
    public const int ERR_INTERNAL = -3;
}

public static unsafe class CalcApi
{
    static CalcApi()
    {
        int version = CalcNativeGen.scaffold_calc_abi_version();
        if (version != CalcNativeGen.ABI_VERSION)
        {
            throw new InvalidOperationException(
                $"scaffold-calc.dll ABI version {version}, expected {CalcNativeGen.ABI_VERSION}."
            );
        }
    }

    public static int Add(int a, int b)
    {
        return CalcNativeGen.scaffold_calc_add(a, b);
    }
}

public sealed unsafe class CalcEngine : IDisposable
{
    nuint _handle;
    bool _disposed;

    public CalcEngine()
    {
        _handle = CalcNativeGen.scaffold_calc_engine_create();
        if (_handle == 0)
        {
            throw new InvalidOperationException("scaffold_calc_engine_create failed.");
        }
    }

    public void SetFactor(int factor)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        Check(CalcNativeGen.scaffold_calc_engine_set_factor(_handle, factor));
    }

    public long Apply(long input)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        long result = 0;
        Check(CalcNativeGen.scaffold_calc_engine_apply(_handle, input, &result));
        return result;
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        CalcNativeGen.scaffold_calc_engine_destroy(_handle);
        _handle = 0;
    }

    static void Check(int rc)
    {
        if (rc != CalcNativeGen.OK)
        {
            throw new InvalidOperationException($"scaffold-calc native error {rc}.");
        }
    }
}
