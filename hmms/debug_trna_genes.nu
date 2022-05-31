# uses ripgrep (see https://github.com/BurntSushi/ripgrep)
ls *tRNA/metadata/*.txt 
    | get name 
    | each { |f| let l = (rg "^[0-9]+\\." $f | lines | last); $"($l) - ($f)" }
    | where $it =~ '^ -'

# some genes have no/spurious hits so remove these post-hoc.
# deleted these genes:
#  ╭───┬──────────────────────────────────────────────╮
#  │ 0 │  - Bryophyta_tRNA/metadata/trnG_UUC.txt      │
#  │ 1 │  - Lycopodiopsida_tRNA/metadata/trnD_GUA.txt │
#  │ 2 │  - Lycopodiopsida_tRNA/metadata/trnP_ACG.txt │
#  │ 3 │  - Lycopodiopsida_tRNA/metadata/trnS_GCA.txt │
#  ╰───┴──────────────────────────────────────────────╯