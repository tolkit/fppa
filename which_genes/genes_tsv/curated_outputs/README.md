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

## Remarks

- rpl21 absent from Gymnosperms
- ycf66 absent from Gymnosperms
- cemA absent from hornworts?
- cysA present in hornworts?
- rps15 absent hornworts?
- ycf1,2,66 absent hornworts?
- I added ccsA to bryophytes.
- I added petN to bryophytes (even though there only appear to be 9 records in NCBI, see <a href="https://www.ncbi.nlm.nih.gov/protein/?term=petN%5BAll+Fields%5D+AND+chloroplast%5Bfilter%5D+AND+%22Bryophyta%22%5Borganism%5D+NOT+partial%5BAll+Fields%5D+AND+refseq%5Bfilter%5D">this</a>)
- I added rpoA to bryophytes (not present in all it seems.)
- tufA likely pseudogenised in Lycopodiopsida.
- psaM looks to be lost in Angiosperms.
- rpl21 absent from Angiosperms (like Gymnosperms).
- ycf12,66 absent in Angiosperms.
- ycf15 seems interesting in terms of its function in Angiosperms. Seems to be asbent outside this group. See <a href="https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0059620">this paper</a>.
- cysA/T present only in Liverworts. Sometimes is pseudogenised (see <a href="https://bsapubs.onlinelibrary.wiley.com/doi/abs/10.3732/ajb.1100010?casa_token=_HcYccEjrF0AAAAA:gkDARZMo4Q-keUfAEPHa1jvUXKm6IJDx9kBKcfnVHEm_e_RdXaus_SNvW9gU_zjscsFah_ND77kfCuo">this paper</a>).
- rps16 apparently not in Liverworts.
- ycf68 present in Liverworts.
- psaM lost in ferns.
- ycf66 mostly absent in ferns, so omit (plus there are no fern sequences in NCBI).