ls *_core/*.fasta
    | get name
    | each { |e| 
        let split_path = ($e | path split)
        let dir = ($split_path | get 0)
        let file = ($split_path | get 1 | str replace '.fasta' '_filtered.fasta')

        let out = ([$dir $file] | path join)

        # and we want to remove sequences that are too long/far away from the mean.
        # we can do this by using the mean and standard deviation of the lengths of the sequences.
        let sample = (mmft len $e | from tsv -n | get column3)
        let sqrt_sample_size = ($sample | length | math sqrt)
        let std_err_mean = (($sample | math stddev) / $sqrt_sample_size)
        let mean = ($sample | math avg)
        # we only want those samples < mean + 2 std error
        # add a 1 at the end to capture tiny stderrs and rounds meaning
        # that we are getting less than many of the sequences.
        let cutoff = ((($mean + (2 * $std_err_mean)) | math round) + 1)

        # need to do a bit of precomputation here.
        let number_fasta_records = (mmft len -le $cutoff $e | mmft regex -ir "hypothetical" | mmft num | from tsv -n | get column1.0)
        let max_records = if $number_fasta_records < 1000 && $number_fasta_records > 0 {
           $number_fasta_records
        } else {
            1000
        }

        echo $"($e): Mean: ($mean), stderr: ($std_err_mean), cutoff: ($cutoff)."
        # this removes any wacky hypothetical sequences
        # and if there are more than 1000 records, it will
        # randomly sample 1000 records.
        mmft len -le $cutoff $e | mmft regex -ir "hypothetical" | mmft sample -n $max_records | save $out
        
        # problems here?
        # weirdly my errors disappear when I do this
        sleep 333ms
    }