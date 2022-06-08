# Fast Plant Plastid Annotation

<p align="center">
    <img width="300" height="132" src="https://www.darwintreeoflife.org/wp-content/themes/dtol/dist/assets/gfx/dtol-logo-round.png">
</p>

We can use a set of known genes to annotate a plant plastid genome. The gene set differs slightly between clades of land plant. These genes have been taken from public databases (NCBI), and turned into a set of HMM profiles for each clade of plant (currently supported are Liverworts, Hornworts, Mosses, Ferns, Conifers, and flowering plants). These can then be quickly queried against a genome, without the need for a reference.

## Usage

In order to use `fppa` you will need two things. First you will need to get a binary of the executable `fppa`. 

```bash
# assuming you have rust, if not - https://www.rust-lang.org/
git clone https://github.com/tolkit/fppa/
cd fppa
# install to PATH
cargo install --path=.
# bring up the help
fppa -h

# or install to ./target/release
cargo build --release
# bring up the help
./target/release/fppa -h
```

```txt
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
                        https://github.com/tolkit/fppa/tree/main/hmms
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
```

Secondly you will need a copy of the HMMs for each plant clade. If you `git clone`'d this repository, they are under the `./hmms` directory. You can of course make your own HMMs, but the names will need to match the ones in the `hmms` directory, otherwise `fppa` will complain.

An example command:

```bash
# the executable
fppa \
# point to the chloroplast genome
--plant-chloro ./genomes/daBalNigr1_segs.fasta \
# point to the nhmmer executable
--nhmmer-path ~/bin/nhmmer \
# point to the directory containing the HMMs (here we're interested in flowering plants)
--hmms-path ./hmms/Magnoliopsida/ \
# generate an HTML SVG of the annotated genes
--plot test.html \
# output a GFF3 file of gene locations
--gff test.gff \
# make a more stringent E value cut-off
--e-value 0.00000001
```

## Output

Output depends on your options. By default a TSV will be printed to STDOUT which shows the presence/absence of each gene in the genome. It's possible to add `--plot` to generate an HTML file of the annotated genes, and with the `--gff` option a GFF3 file will be generated.

## Reqiurements

You will need the latest version of <a href="http://hmmer.org/">HMMER</a>, either in your PATH, or you can point `fppa` to the binary.

## Disclaimer

This annotator is not supposed to be extremely complete nor accurate. The aim is just to determine the presence/absence of genes which should be present on a plant chloroplast *speedily* and *without hassle*. 