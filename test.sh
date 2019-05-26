for i in {0..3}; do
    target/install/bin/cita stop test-chain/$i
done

for i in {0..3}; do
    target/install/bin/cita start test-chain/$i
done

