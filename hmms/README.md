# Pipeline

```nu
# get all the genes first
nu get_core_genes.nu
nu get_trna_genes.nu
# the latter makes a bunch of metadata we then need to process
nu get_trna_fastas.nu

# we can then make the MSA's
nu make_trna_msas.nu
# the core genes need to be filtered
# as many contain hypothetical proteins

# then we can make the HMM's
nu make_hmms.nu
```

Acrogymnospermae_core:
- aacD re-do (delete some fasta entries.)

Marchantiophyta_core:
- ycf68 deleted