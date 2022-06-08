

# don't want dependencies
cargo doc --no-deps --document-private-items
# remove old docs
rm -rf ./docs
# magic..?
echo "<meta http-equiv=\"refresh\" content=\"0; url=fppa\">" | save --raw target/doc/index.html
# copy to docs
cp -r target/doc ./docs
# run an example
# the assembled malus mitochondria, all in one.
# takes ~ 20 seconds.
./target/release/fppa --plant-chloro ./genomes/daBalNigr1.fasta --nhmmer-path ~/bin/nhmmer  --hmms-path ./hmms/Magnoliopsida/ --plot chloro.html --e-value 0.0000000000001
# move it to the docs.
mv ./chloro.html ./docs/fppa