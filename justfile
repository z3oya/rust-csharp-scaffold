native:
    cargo build --release

winforms: native
    dotnet run --project apps/winforms

wpf: native
    dotnet run --project apps/wpf

test:
    cargo test --workspace

fmt:
    cargo fmt --all

check:
    cargo clippy --workspace --all-targets

clean:
    cargo clean
    dotnet clean Scaffold.slnx
