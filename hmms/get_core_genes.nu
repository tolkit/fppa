
# the taxids for each major group.
let taxids = ([{
    Magnoliopsida: 3398
    Acrogymnospermae: 1437180
    Polypodiopsida: 241806
    Lycopodiopsida: 1521260
    Anthocerotophyta: 13809
    Bryophyta: 3208
    Marchantiophyta: 3195
}] | transpose taxon id)

let core_gene_files = (ls ../which_genes/genes_tsv/curated_outputs/*core*.txt | get name)

# iterate over taxids
for $t in $taxids {
    # need to collect into a string
    let gene_file = ($core_gene_files | where $it =~ $"($t.taxon)" | str collect)
    let genes = (open $gene_file | lines)
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

