# Curated outputs

## Process

Made initially by:

```nu
# for example ferns
open ./Polypodiopsida_tRNA.tsv | where count > 30 | get value | save ../curated_outputs/Polypodiopsida_tRNA_curated.txt
# or angiosperms
# slightly more complex to get rid of tRNA's without anticodons.
open ./Magnoliopsida_tRNA.tsv 
    | where value !~ "^trn[a-z]?[A-Z]+$" 
    | where count > 150 
    | get value 
    | save ../curated_outputs/Magnoliopsida_tRNA_curated.txt
# I post sorted stupidly
ls 
    | where name =~ "\\.txt$" 
    | get name 
    | each { |f| let fn = ($f | str collect); (open $f | lines | sort | save $"sorted_($fn)")}

```

Manual edits then made to remove duplicates, or spurious genes.

## Questions

In some clades there are things like:

```txt
rrn4.5
rrn4.5S
```

I deleted the ones with the 'S'.