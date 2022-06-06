# Pipeline

Briefly as follows:

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
nu filter_core_gene_fastas.nu
# then make the core gene msa's
nu make_core_msas.nu

# then we can make the HMM's
nu make_hmms.nu
```

**Warning**
> Warning to future me and anyone else, there were/are plenty of sequences downloaded from NCBI which weren't actually the specified gene/tRNA. I've done my best to filter out where I can, but things will have slipped through the net. My hope is that this will not be a big problem.

Acrogymnospermae_core:
- aacD re-do (delete some fasta entries.) done

Marchantiophyta_core:
- ycf68 deleted

MSA's that might need re-doing:
- Acrogymnospermae_core
  - matK done
  - psbA done
- Magnoliopsida_core
  - accD done (though still a bit wacky)
  - matK done
  - psbB done
  - rpl22 done
  - ycf1
  - ycf2
- Marchantiophyta_core
  - rbcL
- Polypodiopsida_core
  - psbZ
  - ycf1