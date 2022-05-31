ls ./*tRNA/fastas/*.fasta 
    | get name
    | each {
        |e| let name = ($e | path split)
        let new_path = ([($name | get 0) ($name | get 1) ($name | get 2 | str replace '.fasta' '.fna')] | path join)

        /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft $e | save $new_path
    }
    