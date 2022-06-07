# conifers
let acrogymnospermae_core = (open genes_tsv/curated_outputs/Acrogymnospermae_core_curated.txt 
    | lines)

let acrogymnospermae_trna = (open genes_tsv/curated_outputs/Acrogymnospermae_tRNA_curated.txt 
    | lines)


let acrogymnospermae = ($acrogymnospermae_core 
    | append $acrogymnospermae_trna 
    | dfr to-df 
    | dfr rename "Acrogymnospermae")

# hornworts
let anthocerotophyta_core = (open genes_tsv/curated_outputs/Anthocerotophyta_core_curated.txt 
    | lines)

let anthocerotophyta_trna = (open genes_tsv/curated_outputs/Anthocerotophyta_tRNA_curated.txt 
    | lines)


let anthocerotophyta = ($anthocerotophyta_core 
    | append $anthocerotophyta_trna 
    | dfr to-df 
    | dfr rename "Anthocerotophyta")

# marchantiophyta
let marchantiophyta_core = (open genes_tsv/curated_outputs/Marchantiophyta_core_curated.txt 
    | lines)

let marchantiophyta_trna = (open genes_tsv/curated_outputs/Marchantiophyta_tRNA_curated.txt 
    | lines)


let anthocerotophyta = ($anthocerotophyta_core 
    | append $anthocerotophyta_trna 
    | dfr to-df 
    | dfr rename "Anthocerotophyta")

# mosses
let bryophyta_core = (open genes_tsv/curated_outputs/Bryophyta_core_curated.txt 
    | lines)

let bryophyta_trna = (open genes_tsv/curated_outputs/Bryophyta_tRNA_curated.txt 
    | lines)


let bryophyta = ($bryophyta_core 
    | append $bryophyta_trna 
    | dfr to-df 
    | dfr rename "Bryophyta")

# lycopods
let lycopodiopsida_core = (open genes_tsv/curated_outputs/Lycopodiopsida_core_curated.txt 
    | lines)

let lycopodiopsida_trna = (open genes_tsv/curated_outputs/Lycopodiopsida_tRNA_curated.txt 
    | lines)


let lycopodiopsida = ($lycopodiopsida_core 
    | append $lycopodiopsida_trna 
    | dfr to-df 
    | dfr rename "Lycopodiopsida")

# angiosperms
let magnoliopsida_core = (open genes_tsv/curated_outputs/Magnoliopsida_core_curated.txt 
    | lines)

let magnoliopsida_trna = (open genes_tsv/curated_outputs/Magnoliopsida_tRNA_curated.txt 
    | lines)


let magnoliopsida = ($magnoliopsida_core 
    | append $magnoliopsida_trna 
    | dfr to-df 
    | dfr rename "Magnoliopsida")

# ferns
let polypodiopsida_core = (open genes_tsv/curated_outputs/Polypodiopsida_core_curated.txt 
    | lines)

let polypodiopsida_trna = (open genes_tsv/curated_outputs/Polypodiopsida_tRNA_curated.txt 
    | lines)


let polypodiopsida = ($polypodiopsida_core 
    | append $polypodiopsida_trna 
    | dfr to-df 
    | dfr rename "Polypodiopsida")

$polypodiopsida_core 
    | append $polypodiopsida_trna
    | sort
    | each { |e| let f = ($e | str replace '\(' '_' | str replace '\)' '' | str replace '\.5' ''); print $"($f)," }

# display
# $acrogymnospermae_core 
#     | append $acrogymnospermae_trna 
#     | append $bryophyta_core 
#     | append $bryophyta_trna 
#     | append $lycopodiopsida_core 
#     | append $lycopodiopsida_trna 
#     | append $magnoliopsida_core 
#     | append $magnoliopsida_trna 
#     | append $polypodiopsida_core 
#     | append $polypodiopsida_trna 
#     | append $anthocerotophyta_core 
#     | append $anthocerotophyta_trna 
#     | sort | uniq
#     | each {
#         |e| print $"($e | str replace '\(' '_' | str replace '\)' '' | str replace '\.5' ''),"
    # }
# $bryophyta
# $lycopodiopsida 
# $magnoliopsida 
# $polypodiopsida 
