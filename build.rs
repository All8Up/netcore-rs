use std::env;
use std::path::Path;
use std::process::Command;

// dotnet publish -f netcoreapp3.1 --self-contained -r win10-x64
// %SRCDIR%\ManagedLibrary\ManagedLibrary.csproj -o %OUTDIR%

fn main()
{
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    {
        let source_file = Path::new(&crate_dir).join("tests/ManagedLibrary/ManagedLibrary.csproj");
        let dest_path = Path::new(&crate_dir).join("tests/ManagedLibrary/deploy");

        Command::new("dotnet").args(&[
            "publish",
            "-f", "netcoreapp3.1",
            "--self-contained",
            "-r", "win10-x64",
            &(source_file.to_string_lossy().to_string()),
            "-o", &dest_path.to_string_lossy().to_string(),
        ])
        .status().expect("Failed to run deployment.");
    }

    {
        let source_file = Path::new(&crate_dir).join("tests/Avalonia/AvaloniaTestApp.csproj");
        let dest_path = Path::new(&crate_dir).join("tests/Avalonia/deploy");

        Command::new("dotnet").args(&[
            "publish",
            "-f", "netcoreapp3.1",
            "--self-contained",
            "-r", "win10-x64",
            &(source_file.to_string_lossy().to_string()),
            "-o", &dest_path.to_string_lossy().to_string(),
        ])
        .status().expect("Failed to run deployment.");
    }
}
