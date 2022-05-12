
# the taxids for each major group.
let taxids = ([{
    angiosperms: 3398
    gymnosperms: 1437180
    ferns: 241806
    lycopods: 1521260
    hornworts: 13809
    mosses: 3208
    liverworts: 3195
}] | transpose taxon id)

# and the core genes. See the ../which_genes dir 
# for more details.
# seem to be ~ 83 which are not trna's/rrna's

let genes = [
                "rps12",
                "ndhB",
                "rps7",
                "rpl23",
                "rpl2",
                "ycf2",
                "ycf1",
                "rps19",
                "rps15",
                "psbA",
                "rbcL",
                "rps3",
                "rpl16",
                "psbB",
                "rpl14",
                "ndhH",
                "rps8",
                "psaC",
                "petD",
                "rps11",
                "cemA",
                "ccsA",
                "rpoA",
                "matK",
                "psbH",
                "psbF",
                "psaI",
                "petA",
                "rpl20",
                "psbT",
                "psbE",
                "psbD",
                "psaB",
                "psaA",
                "petG",
                "atpA",
                "rps4",
                "rpoC1",
                "rpoB",
                "psbK",
                "petN",
                "ndhE",
                "atpI",
                "atpF",
                "atpE",
                "rps14",
                "rpoC2",
                "psbC",
                "petL",
                "petB",
                "atpB",
                "rps18",
                "rpl36",
                "psbM",
                "psbJ",
                "psaJ",
                "ndhG",
                "ndhF",
                "psbI",
                "ndhA",
                "clpP",
                "atpH",
                "rps2",
                "rpl33",
                "ndhJ",
                "ndhD",
                "ndhI",
                "ycf4",
                "ycf3",
                "ndhC",
                "rpl22",
                "psbN",
                "ndhK",
                "psbL",
                "accD",
                "psbZ",
                "rps16",
                "rpl32",
                "ycf15",
                "infA",
                "ycf68",
                "lhbA",
                "orf42",
                "pbf1"
            ]

# iterate over taxids
for $t in $taxids {
    for $g in $genes {
        print $"Searching in taxid: ($t.id) for gene: ($g)."
    }
}

# # get the web key
# let web_key = (fetch "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=protein&term=ycf2[All+Fields]+AND+txid3398[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&retmax=2000&usehistory=y"
#     | get eSearchResult.children.WebEnv.4.children
#     | str collect)

# echo $web_key
# echo $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=ycf2[All+Fields]+AND+txid3398[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"

# fetch $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=ycf2[All+Fields]+AND+txid3398[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"

