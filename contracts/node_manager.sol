pragma solidity ^0.4.11;

import "./strings.sol";
import "./node_interface.sol";


contract NodeManager is NodeInterface {
    using strings for *;

    // the enum prob: default: Close
    enum NodeStatus { Close, Ready, Start }

    // the status of the node: ready, start, not in list/close, maybe there is more
    mapping(address => NodeStatus) public status;
    // admin
    mapping (address => bool) admins;

    // consensus node list
    address[] nodes;

    event NewNode(address _node);
    event ApproveNode(address _node);
    event DeleteNode(address _node);
    event AddAdmin(address indexed _node, address indexed _sender);

    modifier onlyAdmin {
        if (admins[msg.sender]) {
            _;
        } else {
            revert();
        }
    }
    // setup
    function NodeManager(address[] _nodes, address[] _admins) {
        // initialize the address to Start
        for (uint i = 0; i < _nodes.length; i++) {
            status[_nodes[i]] = NodeStatus.Start;
            nodes.push(_nodes[i]);
        }
        // initialize the address of admins
        for (uint j = 0; j < _admins.length; j++)
            admins[_nodes[j]] = true;
    }

    function addAdmin(address _node) onlyAdmin returns (bool) {
        admins[_node] = true;
        AddAdmin(_node, msg.sender);
        return true;
    }

    // apply to be consensus node. status will be ready
    function newNode(address _node) returns (bool) {
        // should not add the started node, what about the already added node
        // require(status[_node] == NodeStatus.Close);
        if (status[_node] == NodeStatus.Ready || status[_node] == NodeStatus.Start) {
            return false; 
        }

        status[_node] = NodeStatus.Ready;
        NewNode(_node);
        // test
        // assert(status[_node] == NodeStatus.Ready);
        return true;
    }

    // approve to be consensus node. status will be start
    function approveNode(address _node) onlyAdmin returns (bool) {
        // the status should be ready
        // require(status[_node] == NodeStatus.Ready);
        if (status[_node] != NodeStatus.Ready) {
            return false;
        }

        status[_node] = NodeStatus.Start;
        nodes.push(_node);
        ApproveNode(_node);
        // assert(status[_node] == NodeStatus.Start);
        return true;
    }

    // delete the consensus node from the list 
    // which means delete the node whoes status is Start
    function deleteNode(address _node) onlyAdmin returns (bool) {
        // require(status[_node] == NodeStatus.Start);
        if (status[_node] != NodeStatus.Start) {
            return false;
        }

        var index = nodeIndex(_node);
        // not found
        if (index >= nodes.length) {
            return false;
        }

        status[_node] = NodeStatus.Close;
        // remove the gap
        for (uint i = index; i < nodes.length - 1; i++) {
            nodes[i] = nodes[i + 1];
        }
        // also delete the last element
        delete nodes[nodes.length - 1];
        nodes.length--;
        DeleteNode(_node);
        // assert(status[_node] == NodeStatus.Close);
        return true;
    }

    // list the node of the Start
    function listNode() constant returns (string) {
        return concatNodes(nodes);
    }

    // get the status of the node
    function getStatus(address _node) constant returns (uint8) {
        return uint8(status[_node]);
    }

    function isAdmin(address _node) constant returns (bool) {
        return admins[_node];
    }

    // interface: link address to a long string
    function concatNodes(address[] _add) internal returns (string nodeList) {        
        if (_add.length > 0) {
            nodeList = toString(_add[0]);
        }

        for (uint i = 1; i < _add.length; i++) {
            nodeList = nodeList.toSlice().concat(toString(_add[i]).toSlice());
        }
    }

    // interface: get the index in the nodes_of_start array
    function nodeIndex(address _node) internal returns (uint) {
        // find the index of the member 
        for (uint i = 0; i < nodes.length; i++) {
            if (_node == nodes[i]) {
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
