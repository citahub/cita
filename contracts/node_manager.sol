pragma solidity ^0.4.11;

import "./strings.sol";
import "./node_interface.sol";

// ignore the permission
contract NodeManager is NodeInterface {
    using strings for *;

    // the enum prob: default: Close
    enum NodeStatus { Close, Ready, Start }

    // the status of the node: ready, start, not in list/close, maybe there is more
    mapping(address => NodeStatus) public status;

    // array for querying the consensus node list
    address[] nodes_of_start; 

    // store the linked string of start
    string node_list;

    event NewNode(address _node);
    event ApproveNode(address _node);
    event DeleteNode(address _node);
    event ListNode(string _linked);
    event GetStatus(address _node);

    // setup
    function NodeManager(address[] _nodes) {
        // initialize the address to Start
        for (uint i = 0; i < _nodes.length; i++) {
            status[_nodes[i]] = NodeStatus.Start;
            nodes_of_start.push(_nodes[i]);
        }
    }

    // apply to be consensus node. status will be ready
    function newNode(address _node) returns (bool) {
        // should not add the started node, what about the already added node
        // require(status[_node] == NodeStatus.Close);
        if (status[_node] == NodeStatus.Ready) {
            NewNode(_node);
            return false; 
        }

        require(status[_node] != NodeStatus.Start);
        status[_node] = NodeStatus.Ready;
        NewNode(_node);
        // test
        // assert(status[_node] == NodeStatus.Ready);
        return true;
    }

    // approve to be consensus node. status will be start
    function approveNode(address _node) returns (bool) {
        // the status should be ready
        // require(status[_node] == NodeStatus.Ready);
        if (status[_node] != NodeStatus.Ready) {
            ApproveNode(_node);
            return false;
        }

        status[_node] = NodeStatus.Start;
        nodes_of_start.push(_node);
        ApproveNode(_node);
        // assert(status[_node] == NodeStatus.Start);
        return true;
    }

    // delete the consensus node from the list 
    // which means delete the node whoes status is Start
    function deleteNode(address _node) returns (bool) {
        // require(status[_node] == NodeStatus.Start);
        if (status[_node] != NodeStatus.Start) {
            DeleteNode(_node);
            return false;
        }

        status[_node] = NodeStatus.Close;
        // also delete it in the array 
        // not fount
        if (node_index(_node) == nodes_of_start.length){
            DeleteNode(_node);
            return false;
        }

        delete nodes_of_start[node_index(_node)];
        DeleteNode(_node);
        // assert(status[_node] == NodeStatus.Close);
        return true;
    }

    // list the node of the Start
    function listNode()  constant returns (string) {
        // link all the address to a string
        concat_nodes(nodes_of_start); 
        ListNode(node_list);
        return node_list;
    }

    // get the status of the node
    function getStatus(address _node) constant returns (uint8) {
        GetStatus(_node);
        return uint8(status[_node]);
    }

    // interface: link address to a long string
    function concat_nodes(address[] _add) internal {
        delete node_list;
        
        // setup
        if (_add.length > 0) {
            node_list = toString(_add[0]);
            ListNode(node_list);
        }

        for(uint i = 1; i < _add.length; i++) {
            node_list = node_list.toSlice().concat(toString(_add[i]).toSlice());
            ListNode(node_list);
        }
    }

    // interface: get the index in the nodes_of_start array
    function node_index(address _node) internal returns (uint) {
        // find the index of the member 
        for (uint i = 0; i < nodes_of_start.length; i++) {
           if (_node == nodes_of_start[i]) {
               return i;
           }
        }
        // if i == length, means not find
        return i;
    }

    // interface: address to string 
    // the returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++) {
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));
        }

        return string(b);
    }
}
