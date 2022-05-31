# fetch up to 2000 fastas
# for each gene in each taxon group
# (these lists are subtly different)

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

# make the directories
for $t in $taxids {
    mkdir $"($t.taxon)_core"
}

let core_gene_files = (ls ../which_genes/genes_tsv/curated_outputs/*core*.txt | get name)

# iterate over taxids
for $t in $taxids {
    # need to collect into a string
    let gene_file = ($core_gene_files | where $it =~ $"($t.taxon)" | str collect)
    let genes = (open $gene_file | lines)
    for $g in $genes {
        print $"Fetching gene fasta: ($g) for taxid: ($t.id)."
        let web_key = (fetch $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=protein&term=($g)[All+Fields]+AND+txid($t.id)[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&retmax=2000&usehistory=y"
                        | get eSearchResult.children.WebEnv.4.children
                        | str collect)
        print $"With Web Key: ($web_key)."
        print $"And search URL: https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=($g)[All+Fields]+AND+txid($t.id)[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"
        
        # the actual fetch
        fetch $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=($g)[All+Fields]+AND+txid($t.id)[organism]+NOT+partial[All+Fields]+AND+chloroplast[filter]+AND+refseq[filter]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"
            | save $"./($t.taxon)/($g).fasta"
        # don't overload the servers
        sleep 333ms
    }
}

