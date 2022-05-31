let trna_meta_files = (ls ./*tRNA/metadata/*.txt | get name)

# let's make the directories
for $file in $trna_meta_files {
    let split_path = ($file | path split);
    let path = ([($split_path | get 0) "fastas"] | path join)
    mkdir $path
}

# make the TSV's which will be queried to make the
# fasta files
for $file in $trna_meta_files {
    open $file
        | lines
        | where $it =~ 'Annotation' 
        | split column ' ' a del acc range com 
        | select acc range com
        | par-each { |e|
            let ranges = ($e.range | str replace '\(' ''| str replace '\)' '' | str replace ',' '' | split column '..' from to)
            let complement = if ($e.com | empty?) {
                'no-complement'
            } else {
                ($e.com | str replace '\)' '')
            }

            let split_path = ($file | path split)
            let path = ([($split_path | get 0) "fastas" (echo ($split_path | get 2 | split row '.' | get 0) ".fasta" | str collect)] | path join)

            # make the parameters for the fetch more explicit
            let accession = ($e.acc)
            let from = ($ranges | get from | str collect)
            let to = ($ranges | get to | str collect)
            # Strand of DNA to retrieve. Available values are "1" for the plus strand and "2" for the minus strand.
            let strand_int = if $complement == 'no-complement' {
                1
            } else {
                2
            }

            print $"Processing ($accession): from - ($from), to - ($to). Strand = ($strand_int)"
            fetch $"https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id=($accession)&strand=($strand_int)&seq_start=($from)&seq_stop=($to)&rettype=fasta&retmode=text&usehistory=y" | save $path --append
            # no server overloads
            sleep 333ms
        }
}
