//! `fppa` ia a small cli/library to parse and plot nhmmer.table files
//! generated by the flag `--tblout` on nhmmer.
//!
//! See the `plot` module for plotting details.
//!
//! See the `cli` module for the command line interface details.
//!
//! The `gff` module creates GFF3 files from the nhmmer.table files.
//!

use std::{
    fmt::{Display, Formatter},
    fs::{self},
    io::{self, BufRead, Write},
    path::PathBuf,
    process::Command,
    str::FromStr,
};

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use tempdir::TempDir;

/// A lightweight GFF3 writer.
pub mod gff;

/// A module to parse command line args.
pub mod cli;

/// A module to plot the output of nhmmer (HMMER3).
pub mod plot;

/// Entry point for running hmmer.
///
/// Takes a mitochondrial genome path, the path to executable
/// nhmmer, and path to the generated HMM's (in this repo it's
/// "./fastas/hmms/")
///
pub fn run_hmmer(
    mitochondrial_genome_path: PathBuf,
    nhmmer_exec_path: PathBuf,
    hmm_path: PathBuf,
    tmp_dir: &TempDir,
    for_rotation: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // the dir where the HMMs live.
    let hmms = fs::read_dir(hmm_path)?;

    // filter for psbA and ycf1 if specified.;
    // if we want only those two genes for rotation
    let hmm_vec = if for_rotation {
        hmms.filter(|x| {
            let name = x.as_ref().unwrap().file_name().into_string().unwrap();
            name.contains("psbA.hmm") || name.contains("ycf1.hmm")
        })
        .collect::<Result<Vec<_>, _>>()?
    } else {
        // we keep all the genes.
        hmms.collect::<Result<Vec<_>, _>>()?
    };

    for hmm in hmm_vec {
        // sort out the HMM path
        let hmm_path = hmm.path();
        eprintln!(
            "[+]\tRunning nhmmer with HMM: {}",
            hmm_path.clone().display()
        );

        let hmm_name = hmm_path.file_name().unwrap().to_str().unwrap();

        // get basename of mitochondrial_genome_path
        let bn_mgp = mitochondrial_genome_path
            .as_path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        // the name of *this* table.
        let hmm_table = format!(
            "{}/{}-{}.table",
            tmp_dir.path().as_os_str().to_str().unwrap(),
            hmm_name,
            bn_mgp
        );

        let output = Command::new(&nhmmer_exec_path)
            .arg("--tblout")
            .arg(hmm_table)
            .arg(hmm_path)
            .arg(&mitochondrial_genome_path)
            .output()
            .expect("Failed to execute process.");

        assert!(output.status.success());
    }

    Ok(())
}

#[derive(Debug, Display, EnumString)]
pub enum Clade {
    Acrogymnospermae,
    Anthocerotophyta,
    Bryophyta,
    Lycopodiopsida,
    Magnoliopsida,
    Marchantiophyta,
    Polypodiopsida,
}

impl Clade {
    pub fn get_gene_set(&self) -> Vec<String> {
        match self {
            Clade::Acrogymnospermae => AcrogymnospermaeGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Anthocerotophyta => AnthocerotophytaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Bryophyta => BryophytaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Lycopodiopsida => LycopodiopsidaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Magnoliopsida => MagnoliopsidaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Marchantiophyta => MarchantiophytaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            Clade::Polypodiopsida => PolypodiopsidaGenes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

/// An enumeration of all the Acrogymnospermae genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum AcrogymnospermaeGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    chlB,
    chlL,
    chlN,
    clpP,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psaM,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps16,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_GGG,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_UCU,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_GGU,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    trnfM_CAU,
    ycf1,
    ycf12,
    ycf2,
    ycf3,
    ycf4,
}

/// An enumeration of all the Anthocerotophyta genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum AnthocerotophytaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    chlB,
    chlL,
    chlN,
    clpP,
    cysA,
    infA,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psaM,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl21,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps16,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_GGG,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_UCU,
    trnS_CGA,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_GGU,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    ycf12,
    ycf3,
    ycf4,
}

/// An enumeration of all the Bryophyta genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum BryophytaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    chlB,
    chlL,
    chlN,
    clpP,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psaM,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl21,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_GGC,
    trnG_UCC,
    trnG_UUC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnI_UAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_GGG,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_UCU,
    trnS_CGA,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_CGU,
    trnT_GGU,
    trnT_UGU,
    trnV_AAC,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    trnfM_CAU,
    trnl_CAU,
    trnl_GAU,
    ycf1,
    ycf12,
    ycf2,
    ycf3,
    ycf4,
    ycf66,
}

/// An enumeration of all the Lycopodiopsida genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum LycopodiopsidaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    chlB,
    chlL,
    chlN,
    clpP,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psaM,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl21,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps16,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GTC,
    trnD_GUA,
    trnD_GUC,
    trnD_GUG,
    trnE_TTC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GTG,
    trnH_GUG,
    trnI_CAT,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_TAG,
    trnL_UAA,
    trnL_UAG,
    trnM_CAT,
    trnM_CAU,
    trnN_GTT,
    trnN_GUU,
    trnP_ACG,
    trnP_GGG,
    trnP_TGG,
    trnP_UGG,
    trnQ_CTG,
    trnQ_TTG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_TCT,
    trnR_UCU,
    trnS_GCA,
    trnS_GCT,
    trnS_GCU,
    trnS_GGA,
    trnS_TGA,
    trnS_UGA,
    trnT_GGT,
    trnT_GGU,
    trnT_TGT,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GTA,
    trnY_GUA,
    trnfM_CAT,
    trnfM_CAU,
    ycf1,
    ycf12,
    ycf2,
    ycf3,
    ycf4,
    ycf66,
}

/// An enumeration of all the Magnoliopsida genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum MagnoliopsidaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    clpP,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps16,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_UCU,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_GGU,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    trnfM_CAU,
    ycf1,
    ycf15,
    ycf2,
    ycf3,
    ycf4,
}

/// An enumeration of all the Marchantiophyta genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum MarchantiophytaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    chlB,
    chlL,
    chlN,
    clpP,
    cysA,
    cysT,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psaM,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl21,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UGA,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_UCU,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_GGU,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    trnfM_CAU,
    ycf1,
    ycf12,
    ycf2,
    ycf3,
    ycf4,
    ycf66,
    ycf68,
}

/// An enumeration of all the Polypodiopsida genes.
#[derive(
    Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum PolypodiopsidaGenes {
    accD,
    atpA,
    atpB,
    atpE,
    atpF,
    atpH,
    atpI,
    ccsA,
    cemA,
    chlB,
    chlL,
    chlN,
    clpP,
    infA,
    matK,
    ndhA,
    ndhB,
    ndhC,
    ndhD,
    ndhE,
    ndhF,
    ndhG,
    ndhH,
    ndhI,
    ndhJ,
    ndhK,
    petA,
    petB,
    petD,
    petG,
    petL,
    petN,
    psaA,
    psaB,
    psaC,
    psaI,
    psaJ,
    psbA,
    psbB,
    psbC,
    psbD,
    psbE,
    psbF,
    psbH,
    psbI,
    psbJ,
    psbK,
    psbL,
    psbM,
    psbN,
    psbT,
    psbZ,
    rbcL,
    rpl14,
    rpl16,
    rpl2,
    rpl20,
    rpl21,
    rpl22,
    rpl23,
    rpl32,
    rpl33,
    rpl36,
    rpoA,
    rpoB,
    rpoC1,
    rpoC2,
    rps11,
    rps12,
    rps14,
    rps15,
    rps16,
    rps18,
    rps19,
    rps2,
    rps3,
    rps4,
    rps7,
    rps8,
    rrn16,
    rrn23,
    rrn4,
    rrn5,
    trnA_UGC,
    trnC_GCA,
    trnD_GUC,
    trnE_UUC,
    trnF_GAA,
    trnG_GCC,
    trnG_UCC,
    trnH_GUG,
    trnI_CAU,
    trnI_GAU,
    trnK_UUU,
    trnL_CAA,
    trnL_UAA,
    trnL_UAG,
    trnM_CAU,
    trnN_GUU,
    trnP_GGG,
    trnP_UGG,
    trnQ_UUG,
    trnR_ACG,
    trnR_CCG,
    trnR_UCG,
    trnR_UCU,
    trnS_CGA,
    trnS_GCU,
    trnS_GGA,
    trnS_UGA,
    trnT_GGU,
    trnT_UGU,
    trnV_GAC,
    trnV_UAC,
    trnW_CCA,
    trnY_GUA,
    trnfM_CAU,
    ycf1,
    ycf12,
    ycf2,
    ycf3,
    ycf4,
}

/// The strandedness of the HMM hit in the genome.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Strand {
    Positive,
    Negative,
}

/// Implementation of `FromStr` for `Strand`.
impl FromStr for Strand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Strand::Positive),
            "-" => Ok(Strand::Negative),
            _ => Err("The input was neither `-` nor `+`."),
        }
    }
}

/// An implementation of `Display` for `Strand`.
impl Display for Strand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            Strand::Positive => write!(f, "+"),
            Strand::Negative => write!(f, "-"),
        }
    }
}

/// A row representation of the tabular output of nhmmer
/// using the `--tblout` option.
#[derive(Debug)]
#[allow(dead_code)]
pub struct NhmmerRow {
    target_name: String,
    query_name: String,
    hmm_from: i32,
    hmm_to: i32,
    ali_from: i32,
    ali_to: i32,
    env_from: i32,
    env_to: i32,
    sq_len: i32,
    strand: Strand,
    e_value: f32,
    score: f32,
    bias: f32,
}

/// Database representation of the tabular output of
/// nhmmer.
#[derive(Debug)]
pub struct Nhmmer {
    pub rows: Vec<NhmmerRow>,
    pub gene_set: Option<Vec<String>>,
}

impl Nhmmer {
    /// A new `Nhmmer` instance.
    pub fn new() -> Self {
        Self {
            rows: vec![],
            gene_set: None,
        }
    }

    /// In place mutation of `Nhmmer` where tables are read from a
    /// `tempdir::TmpDir` and pushed into the database.
    ///
    /// Critically, the output is then sorted by target name, query name
    /// and then E-value.
    ///
    pub fn read_tables_and_parse(
        &mut self,
        tmp_dir: &TempDir,
        clade: Clade,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // this gets all the files in the table path directory
        let tables = fs::read_dir(tmp_dir)?;

        let gene_set = clade.get_gene_set();
        // make sure we set clade here
        self.gene_set = Some(gene_set.clone());

        for table in tables {
            let table_path = table.expect("Could not open table path").path();

            let table_file = fs::File::open(table_path)?;
            let table_file_lr = io::BufReader::new(table_file).lines();

            for line in table_file_lr {
                let l = line?;
                if l.starts_with('#') {
                    continue;
                }
                let l_vec = l.split_whitespace().collect::<Vec<&str>>();

                let row = NhmmerRow {
                    target_name: l_vec[0].parse::<String>()?,
                    query_name: if gene_set.contains(&l_vec[2].to_string()) {
                        l_vec[2].to_string()
                    } else {
                        // make this nicer.
                        panic!("{} is not in the gene set", l_vec[2])
                    },
                    hmm_from: l_vec[4].parse::<i32>()?,
                    hmm_to: l_vec[5].parse::<i32>()?,
                    ali_from: l_vec[6].parse::<i32>()?,
                    ali_to: l_vec[7].parse::<i32>()?,
                    env_from: l_vec[8].parse::<i32>()?,
                    env_to: l_vec[9].parse::<i32>()?,
                    sq_len: l_vec[10].parse::<i32>()?,
                    strand: l_vec[11].parse::<Strand>()?,
                    e_value: l_vec[12].parse::<f32>()?,
                    score: l_vec[13].parse::<f32>()?,
                    bias: l_vec[14].parse::<f32>()?,
                };

                self.rows.push(row);
            }
        }

        // now order by target name, and query name and e value
        self.rows.sort_by(|a, b| {
            a.target_name.cmp(&b.target_name).then(
                a.query_name
                    .cmp(&b.query_name)
                    .then(a.e_value.partial_cmp(&b.e_value).unwrap()),
            )
        });

        Ok(())
    }

    /// Produce a GFF from the NHMMER tables.
    ///
    /// This function is pretty copy-heavy and I *might*
    /// re-factor at some point.
    #[allow(unused_variables)]
    pub fn make_gff3(
        &self,
        path: PathBuf,
        e_value_cli: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut gff3 = gff::GFF3::new();
        // iterate over the rows
        for NhmmerRow {
            target_name,
            query_name,
            hmm_from,
            hmm_to,
            ali_from,
            ali_to,
            env_from,
            env_to,
            sq_len,
            strand,
            e_value,
            score,
            bias,
        } in &self.rows
        {
            if *e_value > e_value_cli {
                continue;
            }
            let query_name = format!("{}", query_name);
            let type_: String = match query_name.starts_with("Trn") {
                true => "tRNA".into(),
                // I think I'm annotating either CDS or exons...
                false => "CDS".into(),
            };

            gff3.add_record(gff::GFF3Field {
                seqid: target_name.clone(),
                source: String::from("fppa"),
                r#type: type_,
                start: *env_from as usize,
                end: *env_to as usize,
                score: *score,
                strand: *strand,
                phase: 0,
                attributes: format!("Name={}", query_name),
            })
        }
        // create the file
        let mut gff_file = fs::File::create(path)?;

        gff::write_gff3(gff3, &mut gff_file)?;

        gff_file.flush()?;

        Ok(())
    }

    /// Returns a `PlotData` object to be plotted if specified.
    ///
    /// The `Nhmmer` database is filtered on E-value threshold,
    /// and then only the top E-value hit for each gene is kept.
    ///
    /// A TSV is printed to STDOUT indicating presence/absence
    /// of genes.
    pub fn filter_table_and_print(
        &mut self,
        e_value: f32,
    ) -> Result<plot::PlotData, Box<dyn std::error::Error>> {
        // filter the rows on some E - value threshold
        self.rows.retain(|row| row.e_value < e_value);

        // collect the hits from each fasta into a separate vec
        // EDIT: no longer filter out secondary hits at the same time.
        // as these show us the exons.
        // this data will eventually be plotted
        let mut plot_data = plot::PlotData::new();
        self.rows.iter().for_each(|e| {
            let group = plot_data
                .data
                .entry(e.target_name.clone())
                .or_insert(vec![]);
            group.push(plot::PlotDataRow {
                query_name: e.query_name.clone(),
                env_from: e.env_from,
                env_to: e.env_to,
                strand: e.strand,
                e_value: e.e_value,
                seq_len: e.sq_len,
            });
        });

        // print completeness data
        // potentially to be made optional but hey-ho.
        plot_data.completeness(self.gene_set.clone().unwrap());

        let headers: Vec<String> = plot_data.data.iter().map(|(k, _)| k.clone()).collect();

        println!("Chlorogene\t{}", headers.join("\t"));

        for chlorogene in self.gene_set.clone().unwrap() {
            // we want to know if the gene is present for each contig
            // BTreeMap so keys should stay in order.
            let is_present: Vec<bool> = plot_data
                .data
                .iter()
                .map(|(_, v)| {
                    v.iter()
                        .map(|e| e.query_name.clone())
                        .any(|x| x == *chlorogene)
                })
                .collect();

            print!("{}\t", chlorogene);
            for e in is_present {
                match e {
                    true => print!("true\t"),
                    false => print!("false\t"),
                }
            }
            println!()
        }

        Ok(plot_data)
    }
}