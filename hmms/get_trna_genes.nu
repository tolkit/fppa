# e.g. 
# trnA UGC search
# for https://www.ncbi.nlm.nih.gov/gene?term=trnA%5BAll%20Fields%5D%20AND%20UGC%5BAll%20Fields%5D%20AND%20%22Magnoliopsida%22%5Borganism%5D%20AND%20%28%22source%20plastid%22%5Bproperty%5D%29&cmd=DetailsSearch

# fetch up to 2000 fastas
# for each gene in each taxon group
# (these lists are subtly different)

# the tRNA pipeline differs as you can't fetch the proteins  (duh).
# 1. Fetch the metadata
# 2. 

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
    mkdir $"($t.taxon)_tRNA/metadata/"
}

let trna_gene_files = (ls ../which_genes/genes_tsv/curated_outputs/*tRNA*.txt | get name)

# iterate over taxids
for $t in $taxids {
    # need to collect into a string
    let gene_file = ($trna_gene_files | where $it =~ $"($t.taxon)" | str collect)
    let genes = (open $gene_file | lines)
    # iterate over the genes
    for $g in $genes {
        let parse_gene = (get_gene_and_anticodon $g)

        # annoyingly, some genes within the tRNA bucket dont 
        # have anticodons (e.g. rrn16)
        if ($parse_gene | length) == 2 {
            let gene = ($parse_gene | get 0)
            let anticodon = ($parse_gene | get 1)

            # initial metadata fetches
            let web_key = (curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=gene&term=txid($t.id)[Organism]+AND+($gene)[All%20Fields]+AND+($anticodon)[All%20Fields]+AND+%28%22source%20plastid%22[property]&retmax=2000&usehistory=y" 
                            | from xml 
                            | get eSearchResult.children.WebEnv.4.children 
                            | str collect)
            print $"Fetching gene: ($gene) with anticodon: ($anticodon), for taxid: ($t.id). Web Key = ($web_key)"

            # save in the metadata subfolder, with the gene & anticodon.
            curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=gene&term=txid($t.id)[Organism]+AND+($gene)[All%20Fields]+AND+($anticodon)[All%20Fields]+AND+%28%22source%20plastid%22[property]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"
                | save $"($t.taxon)_tRNA/metadata/($gene)_($anticodon).txt"

            sleep 333ms
        } else {
            let gene = ($parse_gene | get 0)

            # initial metadata fetches
            let web_key = (curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=gene&term=txid($t.id)[Organism]+AND+($gene)[All%20Fields]+AND+%28%22source%20plastid%22[property]&retmax=2000&usehistory=y" 
                            | from xml 
                            | get eSearchResult.children.WebEnv.4.children 
                            | str collect)

            print $"Fetching gene: ($gene) without anticodon, for taxid: ($t.id). Web Key = ($web_key)."

            # save in the metadata subfolder, with the gene & anticodon.
            curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=gene&term=txid($t.id)[Organism]+AND+($gene)[All%20Fields]+AND+%28%22source%20plastid%22[property]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"
                | save $"($t.taxon)_tRNA/metadata/($gene).txt"
            sleep 333ms
        }
    }
}

# get gene out of a string of:
# gene(anticodon)
def get_gene_and_anticodon [
    input: string
] {
    $input | split row "(" | split row ")"
}

# this works! "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=gene&term=trnS%5BAll%20Fields%5D+AND+UGA%5BAll%20Fields%5D+AND+%22Acrogymnospermae%22%5Borganism%5D&retmax=2000&usehistory=y"
# curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=gene&term=txid3398[Organism]+AND+trnA[All%20Fields]+AND+UGC[All%20Fields]+AND+%28%22source%20plastid%22[property]&retmax=2000&usehistory=y"
# curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=gene&term=txid($t.id)[Organism]+AND+($gene)[All%20Fields]+AND+($anticodon)[All%20Fields]+AND+%28%22source%20plastid%22[property]&rettype=fasta&retmode=text&WebEnv=($web_key)&query_key=1"