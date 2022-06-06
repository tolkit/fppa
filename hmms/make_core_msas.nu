# pipeline to generate a multiple sequence
# alignment from all of our core genes.
ls *_core/*_filtered.fasta
    | get name
    | each { |e| 
        let split_path = ($e | path split)
        let dir = ($split_path | get 0)
        let file = ($split_path | get 1 | str replace '.fasta' '.fna')

        let out = ([$dir $file] | path join)

        /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft --auto --thread 4 $e | aa2nucaln | save $out
        # problems here?
        sleep 333ms
        }

# run on all of the core genes where there were no filtered fasta files

# ls *_core/*_filtered.fasta | where size == 0B | get name | str replace '_filtered' '' | each { |e| let split_path = ($e | path split); let dir = ($split_path | get 0); let file = ($split_path | get 1| str replace '.fasta' '.fna'); let out = ([$dir $file] | path join); aa2nucaln --fasta $e | /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft --auto --thread 4 - | save $out }