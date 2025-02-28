cargo test
sudo setcap CAP_NET_ADMIN=eip ./target/release/rusty-tcp
./target/release/rusty-tcp &
sudo ip link set tun0 up
sudo ip addr add 192.168.0.1/24 dev tun0
pid=$!
trap "kill $pid" INT TERM
wait $pid
