#!/bin/csh

#---------------------
# RUN ALL CARGO TESTS
#---------------------


# PuffyRS
cd puffyrs
cargo test
cd ..

# bin/echo
cd "bin/echo"
cargo test
cd ..

# bin/chroot
cd "bin/chroot"
cargo test
cd ..
