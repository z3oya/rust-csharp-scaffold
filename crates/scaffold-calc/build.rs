fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("scaffold-calc")
        .csharp_namespace("Scaffold.Bindings")
        .csharp_class_name("CalcNativeGen")
        .csharp_class_accessibility("public")
        .csharp_generate_const_filter(|n| {
            matches!(
                n,
                "OK" | "ERR_INVALID_HANDLE" | "ERR_BAD_ARGUMENT" | "ERR_INTERNAL" | "ABI_VERSION"
            )
        })
        .generate_csharp_file("../../libs/bindings/CalcNativeGen.g.cs")
        .unwrap();
}
