# we want to open each of the gene set files
# and make nuon files of the most common/likely
# candidate gene sets

mkdir genes_tsv
mkdir genes_tsv/raw_outputs

def csv_to_tsv_trna [
        input_csv: path, 
        #count_: int, remove for now
        save_path: path
    ] {
    open $input_csv
        | get genes 
        | str collect ',' 
        | split row ',' 
        | sort 
        | uniq -c 
        | sort-by count -r
        | where value =~ 'trn|tRNA|rrn'
        | save $save_path
}

def csv_to_tsv_core [
        input_csv: path, 
        # count_: int, 
        save_path: path
    ] {
    open $input_csv
        | get genes 
        | str collect ',' 
        | split row ',' 
        | sort 
        | uniq -c 
        | sort-by count -r
        | where value !~ 'trn|tRNA|rrn'
        | save $save_path
}

# gymnosperms
csv_to_tsv_trna './gene_sets/gene_sets_500_Acrogymnospermae.csv' './genes_tsv/raw_outputs/Acrogymnospermae_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Acrogymnospermae.csv' './genes_tsv/raw_outputs/Acrogymnospermae_core.tsv'
# hornworts
csv_to_tsv_trna './gene_sets/gene_sets_500_Anthocerotophyta.csv' './genes_tsv/raw_outputs/Anthocerotophyta_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Anthocerotophyta.csv' './genes_tsv/raw_outputs/Anthocerotophyta_core.tsv'
# mosses
csv_to_tsv_trna './gene_sets/gene_sets_500_Bryophyta.csv' './genes_tsv/raw_outputs/Bryophyta_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Bryophyta.csv' './genes_tsv/raw_outputs/Bryophyta_core.tsv'
# lycopods!
csv_to_tsv_trna './gene_sets/gene_sets_500_Lycopodiopsida.csv' './genes_tsv/raw_outputs/Lycopodiopsida_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Lycopodiopsida.csv' './genes_tsv/raw_outputs/Lycopodiopsida_core.tsv'
# flowers
csv_to_tsv_trna './gene_sets/gene_sets_500_Magnoliopsida.csv' './genes_tsv/raw_outputs/Magnoliopsida_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Magnoliopsida.csv' './genes_tsv/raw_outputs/Magnoliopsida_core.tsv'
# liverworts
csv_to_tsv_trna './gene_sets/gene_sets_500_Marchantiophyta.csv' './genes_tsv/raw_outputs/Marchantiophyta_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Marchantiophyta.csv' './genes_tsv/raw_outputs/Marchantiophyta_core.tsv'
# ferns
csv_to_tsv_trna './gene_sets/gene_sets_500_Polypodiopsida.csv' './genes_tsv/raw_outputs/Polypodiopsida_tRNA.tsv'
csv_to_tsv_core './gene_sets/gene_sets_500_Polypodiopsida.csv' './genes_tsv/raw_outputs/Polypodiopsida_core.tsv'
