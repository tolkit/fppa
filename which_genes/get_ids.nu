# it actually takes a while to download the things.
# and I don't have forever
# 1.5 hours using this return number
# we don't want to overload the eutils servers either
# hence the 500ms delay between requests.

let RETURN_NUMBER = 500
mkdir gene_sets

# enumerate the clades
let clades = [
    "Magnoliopsida", 
    "Acrogymnospermae",
    "Polypodiopsida",
    "Lycopodiopsida",
    "Anthocerotophyta",
    "Bryophyta",
    "Marchantiophyta",
]

# and iterate over them
for clade in $clades {
    # use eutils to get an XML of
    # all chloroplast complete genomes
    # actually should probably use fetch...
    let id_list = (curl -L $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=nuccore&term=\(complete%5BAll+Fields%5D+AND+chloroplast%5BAll+Fields%5D+AND+genome%5BAll+Fields%5D\)+AND+\("($clade)"%5BOrganism%5D\)+AND+chloroplast%5Bfilter%5D&retmax=20000&usehistory=y"
                    | from xml 
                    | get eSearchResult.children)

    # parse the pesky XML
    let ids = ($id_list 
        | get IdList 
        | select 5 
        | get children.0 
        | each { |i| echo $i | get Id.children.0 })

    # save the URLs and shuffle
    # thanks to the handy GFF3 retrieval hack here
    # https://www.biostars.org/p/296825/
    let urls = ($ids | each {
        |e| echo $"https://www.ncbi.nlm.nih.gov/sviewer/viewer.cgi?db=nuccore&report=gff3&id=($e)"
    } | shuffle)

    # return the GFFs and parse them.
    # sample 200 at random.
    # should be enough to get an idea of the genes present!

    let file = $"gene_sets_($RETURN_NUMBER)_($clade).csv"
    touch $file

    $urls | first $RETURN_NUMBER | each -n {
        |e| echo $"Processing item ($e.index + 1) out of ($RETURN_NUMBER) for clade ($clade)."
        
            # fetching twice, could be streamlined.
            let URL = (fetch $e.item
                    | lines
                    | take 2
                    | last 
                    | str substring '10,');
        
            let GENES = (fetch $e.item 
                | lines 
                | skip 2 
                | split column "\t" 
                | take until column1 == "" 
                | where column3 =~ "gene"
                | get column9 
                | split row ";" 
                | each { |i| if $i =~ "^gene=" { echo $i | str substring '5,' } }); 
                
                if $e.index == 0 {
                    [
                        {
                            url: $URL,
                            genes: ($GENES | str collect ','),
                            len: ($GENES | length),
                        }
                    ] | to csv | save $file --append
                } else {
                    # don't want the headers on the subsequent run
                    [
                        {
                            url: $URL,
                            genes: ($GENES | str collect ','),
                            len: ($GENES | length),
                        }
                    ] | to csv --noheaders | save $file --append
                }

            sleep 500ms
        }
}

# move them all to a better place :)
mv $"gene_sets_($RETURN_NUMBER)_*.csv" ./gene_sets