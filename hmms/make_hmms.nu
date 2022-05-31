let taxids = [
    'Magnoliopsida'
    'Acrogymnospermae'
    'Polypodiopsida'
    'Lycopodiopsida'
    'Anthocerotophyta'
    'Bryophyta'
    'Marchantiophyta'
]

for $taxid in $taxids {
    mkdir $taxid
}

ls *_core/*.fasta 
    | get name 
    | each { |e| 
        let split_path = (echo $e | path split)
    }