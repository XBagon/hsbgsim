cargo test --test basic full_annoy -- --nocapture | save -f .\test-output\full_annoy_o_trons.log
cargo test --test basic two_vs_five_annoy -- --nocapture | save -f .\test-output\two_vs_five_annoy_o_trons.log
cargo test --test basic full_random -- --nocapture | save -f .\test-output\full_random.log