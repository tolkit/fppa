use std::path::{Path, PathBuf};

/// The command line help string.
static HELP: &str = "\
<Max Brown; Wellcome Sanger 2022>
Fast plant mito annotation (fppa).
Version: 0.1.0

USAGE:
  fppa --plant-chloro <PATH> --nhmmer-path <PATH> --hmms-path <PATH>
FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version information
ARGS:
  --plant-chloro        Path to the plant chloroplast/plastid genome
  --nhmmer-path         Path to the nhmmer executable (HMMER3)
  --hmms-path           Path to the directory containing a set of
                        HMM files. Download from:
                        https://github.com/tolkit/fppa/hmms/
OPTIONAL ARGS:
  --plot                Generate an HTML SVG of where the annotated
                        genes occur. Requires a name ending in `.html`.
  --e-value             The E-value cut-off determining presence of
                        mito gene. <default 0.001>
  --gff                 Output a GFF3 file of gene locations. Requires
                        a name ending in `.gff`.
  -r, --for-rotation    Identify only psbA and ycf1 for chloroplast 
                        standardisation.
                  
EXAMPLE:
  fppa --plant-chloro ./chloro.fasta --nhmmer-path ./nhmmer --hmms-path ./hmms/angiosperm_hmms/
";

/// A `pico-args` struct to parse the command line args.
#[derive(Debug)]
pub struct AppArgs<P: AsRef<Path>> {
    pub chloroplast_genome: P,
    pub path_to_nhmmer: P,
    pub path_to_hmms: P,
    pub e_value: Option<f32>,
    pub plot: Option<P>,
    pub gff: Option<P>,
    pub for_rotation: bool,
}

/// Parse the command line arguments.
pub fn parse_args() -> Result<AppArgs<PathBuf>, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help, version, and for-rotation have a higher priority.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        print!("fppa version 0.1.0");
        std::process::exit(0);
    }

    let args = AppArgs {
        chloroplast_genome: pargs.value_from_os_str("--plant-chloro", parse_path)?,
        path_to_nhmmer: pargs.value_from_os_str("--nhmmer-path", parse_path)?,
        path_to_hmms: pargs.value_from_os_str("--hmms-path", parse_path)?,
        e_value: pargs.opt_value_from_fn("--e-value", parse_f32)?,
        plot: pargs.opt_value_from_os_str("--plot", parse_path)?,
        gff: pargs.opt_value_from_os_str("--gff", parse_path)?,
        for_rotation: pargs.contains(["-r", "--for-rotation"]),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

/// Parse `OsStr` to `PathBuf`.
fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
    Ok(s.into())
}

/// Parse `&str` to `f32`.
fn parse_f32(s: &str) -> Result<f32, &'static str> {
    s.parse().map_err(|_| "Cannot parse string to f32.")
}
