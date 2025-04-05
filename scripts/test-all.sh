cd modules

for d in */; do
 cd $d
 cargo test
 cd ..
done