using System.Text;

namespace Scaffold.Bindings;

public static unsafe partial class TextNativeGen
{
    public const int ERR_INVALID_HANDLE = -1;
    public const int ERR_BAD_ARGUMENT = -2;
    public const int ERR_INTERNAL = -3;
}

public static unsafe class TextApi
{
    static TextApi()
    {
        int version = TextNativeGen.scaffold_text_abi_version();
        if (version != TextNativeGen.ABI_VERSION)
        {
            throw new InvalidOperationException(
                $"scaffold-text.dll ABI version {version}, expected {TextNativeGen.ABI_VERSION}."
            );
        }
    }

    public static string Greet(string name)
    {
        byte[] bytes = Encoding.UTF8.GetBytes(name);
        fixed (byte* source = bytes)
        {
            ByteBuffer* buffer = TextNativeGen.scaffold_text_greet(source, (uint)bytes.Length);
            if (buffer == null)
            {
                throw new InvalidOperationException("scaffold_text_greet failed.");
            }
            string result = Encoding.UTF8.GetString(buffer->ptr, buffer->length);
            TextNativeGen.scaffold_text_free_byte_buffer(buffer);
            return result;
        }
    }
}
