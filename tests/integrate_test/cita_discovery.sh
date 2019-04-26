#!/bin/bash

get_source_dir() {
    if [[ $(uname) == 'Darwin' ]] ; then
        source_dir=$(realpath "$(dirname "$0")"/../..)
    else
        source_dir=$(readlink -f "$(dirname "$0")"/../..)
    fi
    echo "${source_dir}"
}

# clean up only when it successes
clean_host() {
    sed '/node0/d' /etc/hosts | sudo tee -a /tmp/hosts > /dev/null
    sudo cp /tmp/hosts /etc/hosts
    sudo rm /tmp/hosts
}

set_hosts() {
    # set node0 to /etc/hosts for domain name test
    echo "127.0.0.1    node0" | sudo tee -a /etc/hosts > /dev/null
}

generate_config() {
    ./scripts/create_cita_config.py create \
        --chain_name "node" \
        --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
        --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
        > /dev/null 2>&1

    for i in {1..4} ; do
        ./scripts/create_cita_config.py append \
        --chain_name "node" \
        --node "127.0.0.1:$((4003 + i))" \
        > /dev/null 2>&1
    done

    # node[0..3] keep only 3 peers
    for i in {0..3} ; do
        sed '19, $d' -i node/$i/network.toml
    done

    # node[4..7] keep only 1 peers
    for i in {4..7} ; do
        sed '9, $d' -i node/$i/network.toml
    done
}

pre_start_nodes() {
    # setup for all nodes
    for i in {0..7} ; do
        bin/cita bebop setup node/$i  > /dev/null
    done

    # start node[0..3]
    for i in {0..3} ; do
        bin/cita bebop start node/$i  > /dev/null
    done
}

# T001/T011: discovery node entry
test_node_entry() {
    echo -n "T0$1$2 discovery node entry  ... "

    # start node4
    bin/cita bebop start node/4 > /dev/null

    # check every node's peer count reach to 4
    for i in {0..4} ; do
        timeout=$(check_peer_count $i 4 60)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done

    #stop node4
    bin/cita bebop stop node/4 > /dev/null

    echo "${timeout}s passed"
}

# T002/T012: discovery node exit
test_node_exit() {
    echo -n "T0$1$2 discovery node exit  ... "

    # stop node3
    bin/cita bebop stop node/3 > /dev/null

    # check every node's peer count is 2
    for i in {0..2} ; do
        timeout=$(check_peer_count $i 2 60)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done
    bin/cita bebop start node/3 > /dev/null
    echo "${timeout}s passed"

}

# T003/T013: discovery node parallel entry
test_parallel_entry() {
    echo -n "T0$1$2 discovery nodes for parallel entry  ... "

    # start node[4..7]
    for i in {4..7} ; do
        bin/cita bebop start node/$i > /dev/null
    done

    # check every node's peer count reach to 7
    for i in {0..7} ; do
        # needs more time for 8 nodes discovery each other.
        timeout=$(check_peer_count $i 7 180)||(echo "FAILED"
                                               echo "error msg: ${timeout}"
                                               exit 1)
    done

    echo "${timeout}s passed"
}

# T004/T014: discovery node parallel exit
test_parallel_exit() {
    echo -n "T0$1$2 discovery nodes for parallel exit  ... "

    # start node[4..7]
    for i in {4..7} ; do
        bin/cita bebop stop node/$i > /dev/null
    done

    # check every node's peer count is 3
    for i in {0..3} ; do
        timeout=$(check_peer_count $i 3 60)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done
    echo "${timeout}s passed"
}

# T005/T015: discovery node parallel entry and exit
test_parallel_entry_exit() {
    echo -n "T0$1$2 discovery nodes for parallel entry and exit  ... "

    # start node[4..7] and stop node[1..3]
    bin/cita bebop start node/4 > /dev/null
    for i in {1..3} ; do
        bin/cita bebop stop node/$i > /dev/null
        bin/cita bebop start node/$((i + 4)) > /dev/null
    done

    # stop node[4..7] and start node[1..3]
    bin/cita bebop stop node/4 > /dev/null
    for i in {1..3} ; do
        bin/cita bebop start node/$i > /dev/null
        bin/cita bebop stop node/$((i + 4)) > /dev/null
    done

    # check every node's peer count is 3
    for i in {0..3} ; do
        timeout=$(check_peer_count $i 3 90)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done
    echo "${timeout}s passed"
}

# T006/T016: discovery node entry before known node start
test_entry_before_known_node_start() {
    echo -n "T0$1$2 discovery node for entry before known node start  ... "

    # node0 is the only known node for node4, stop it first.
    bin/cita bebop stop node/0 > /dev/null
    bin/cita bebop start node/4 > /dev/null

    # known nodes start after 10 seconds of node4 startup
    sleep 10

    # start the known node
    bin/cita bebop start node/0 > /dev/null

    # check every node's peer count is 4
    for i in {0..4} ; do
        # needs more time to for checking, because node0 may not have enough score in node4
        timeout=$(check_peer_count $i 4 240)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done
    bin/cita bebop stop node/4 > /dev/null
    echo "${timeout}s passed"
}

# T021: discovery node entry and then known node restart
test_entry_and_known_node_restart() {
    echo -n "T021 discovery node entry and then known node restart  ... "

    # node0 is the only known node for node4
    bin/cita bebop start node/4 > /dev/null

    # check every node0 peer count is 4
    timeout=$(check_peer_count 0 4 90)||(echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    # stop all connected nodes in node4
    for i in {0..3} ; do
        bin/cita bebop stop node/$i > /dev/null
    done
    timeout=$(check_peer_count 4 0 90)||(echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    # start all nodes
    for i in {0..3} ; do
        bin/cita bebop start node/$i > /dev/null
    done

    # check every node's peer count is 4
    for i in {0..4} ; do
        # needs more time to for checking, because node0 may not have enough score in node4
        timeout=$(check_peer_count $i 4 180)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done
    bin/cita bebop stop node/4 > /dev/null
    echo "${timeout}s passed"
}

# T022: discovery node with error address
test_repeated_address() {
    echo -n "T022 discovery node with repeated address  ... "

    # Disguise node4 as node3
    cp node/4/address node/4/address.tmp
    cp node/4/privkey node/4/privkey.tmp

    cp node/3/address node/4/address
    cp node/3/privkey node/4/privkey

    # node0 is the only known node for node4
    bin/cita bebop start node/4 > /dev/null

    # node[0..3]'s peer count is 3
    for i in {0..3} ; do
        timeout=$(check_peer_count $i 3 90)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done

    # node4's peer count is 0
    timeout=$(check_peer_count 4 0 90)||(echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    bin/cita bebop stop node/4 > /dev/null

    # recover the address
    mv node/4/address.tmp node/4/address
    mv node/4/privkey.tmp node/4/privkey

    echo "${timeout}s passed"
}

# T023: discovery node with max connected limit as client
test_max_connected_limit_as_client() {
    echo -n "T023 discovery node with max connected limit as client ... "

    # stop all nodes
    for i in {0..3} ; do
        bin/cita bebop stop node/$i > /dev/null

        # set max_connects = 3
        sed '1 a\max_connects = 3' -i node/$i/network.toml > /dev/null
    done

    # start all nodes
    for i in {0..3} ; do
        bin/cita bebop start node/$i > /dev/null
    done

    # make sure that node[0..3] has been connected each other
    for i in {0..3} ; do
        timeout=$(check_peer_count $i 3 90)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done

    # start node4
    bin/cita bebop start node/4 > /dev/null

    # node[0..3]'s peer count is 3
    for i in {0..3} ; do

        # it is necessary to wait for a few seconds for each check
        sleep 3
        timeout=$(check_peer_count $i 3 90)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done

    # node4's peer count is 0
    timeout=$(check_peer_count 4 0 90)||(echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)

    bin/cita bebop stop node/4 > /dev/null

    # recover the config
    for i in {0..3} ; do
        sed '2d' -i node/$i/network.toml > /dev/null
    done

    echo "${timeout}s passed"
}

# T024: discovery node with max connected limit as server
test_max_connected_limit_as_server() {
    echo -n "T024 discovery node with max connected limit as server ... "

    # stop all nodes
    for i in {0..3} ; do
        bin/cita bebop stop node/$i > /dev/null

        # set max_connects = 2
        sed '1 a\max_connects = 2' -i node/$i/network.toml > /dev/null
    done

    # start all nodes
    for i in {0..3} ; do
        bin/cita bebop start node/$i > /dev/null
    done

    # node[0..3]'s peer count cannot grater than 2,
    # but some nodes may less than 2.
    for i in {0..3} ; do

        # it is necessary to wait for a few seconds for each check
        sleep 3
        timeout=$(check_peer_count_max $i 2 90)||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
    done

    # recover the config
    for i in {0..3} ; do
        sed '2d' -i node/$i/network.toml > /dev/null
    done

    echo "${timeout}s passed"
}

main() {
    set -e

    # FIXME: util needs this two global var, refine later
    SOURCE_DIR=$(get_source_dir)
    BINARY_DIR=${SOURCE_DIR}/target/install

    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . "${SOURCE_DIR}/tests/integrate_test/util.sh"
    cd "${BINARY_DIR}"
    set_hosts
    echo "DONE"

    echo -n "1) cleanup   ...  "
    cleanup
    echo "DONE"

    echo -n "2) generate config  ...  "
    generate_config
    echo "DONE"

    echo -n "3) pre start nodes[0..3]  ...  "
    pre_start_nodes
    echo "DONE"

    # Pre-check peer count, it is OK to check node0 only
    echo -n "4) pre-check peer count  ...  "
    timeout=$(check_peer_count 0 3 60)||(echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    echo "${timeout}s DONE"

    test_suites=("ip" "domain_name")
    index=0

    for test_suit in "${test_suites[@]}" ; do
        if [ "$test_suit" == "domain_name" ] ; then
            # change node[4..7]'s peers to domain name
            for i in {4..7} ; do
                sed 's/127.0.0.1/node0/g' -i node/$i/network.toml
            done
        fi

        test_node_entry ${index} 1
        test_node_exit ${index} 2
        test_parallel_entry ${index} 3

        # this case should just after test_parallel_entry
        test_parallel_exit ${index} 4
        test_parallel_entry_exit ${index} 5
        test_entry_before_known_node_start ${index} 6

        index=$((index+1))
    done

    test_entry_and_known_node_restart
    test_repeated_address
    test_max_connected_limit_as_client
    test_max_connected_limit_as_server

    echo -n "5) cleanup ... "
    clean_host
    cleanup
    echo "DONE"
}

main "$@"
