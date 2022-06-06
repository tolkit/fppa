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

ls *_core/*filtered.fna 
    | get name
    | each { |e|
        let split = ($e | path split)
        let path_to = ($split.0 | str replace '_core' '')
        let file_name = ($split.1 | split column '.' | get 0 | get column1 | str replace '_filtered' '')

        let output_file_name = (([$path_to $file_name ] | path join) + '.hmm')
        echo $"Building ($output_file_name)..."
        
        ~/bin/hmmbuild -n $file_name $output_file_name $e
    }

ls *_tRNA/fastas/*.fna 
    | get name
    | each { |e|
        let split = ($e | path split)
        let path_to = ($split.0 | str replace '_tRNA' '')
        let file_name = ($split | get 2 | str replace '.fna' '')

        let output_file_name = (([$path_to $file_name ] | path join) + '.hmm')
        echo $"Building ($output_file_name)..."
        
        ~/bin/hmmbuild -n $file_name $output_file_name $e
    }