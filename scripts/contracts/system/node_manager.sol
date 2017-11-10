pragma solidity ^0.4.14;

import "./strings.sol";
import "./node_interface.sol";

contract NodeManager is NodeInterface {

    using strings for *;

    mapping(address => NodeStatus) public status;
    mapping (address => bool) admins;
    // Recode the operation of the block
    mapping(uint => bool) block_op;
    // Consensus node list
    address[] nodes;

    // Default: Close
    enum NodeStatus { Close, Ready, Start }

    modifier onlyAdmin {
        require(admins[msg.sender]);
        _;
    }

    // Should operate one time in a block
    modifier oneOperate {
        require(!block_op[block.number]);
        _;
    }

    modifier onlyClose(address _node) {
        require(NodeStatus.Close == status[_node]); 
        _;
    }

    modifier onlyStart(address _node) {
        require(NodeStatus.Start == status[_node]); 
        _;
    }

    modifier onlyReady(address _node) {
        require(NodeStatus.Ready == status[_node]); 
        _;
    }

    /// Setup
    function NodeManager(address[] _nodes, address[] _admins) {
        // Initialize the address to Start
        for (uint i = 0; i < _nodes.length; i++) {
            status[_nodes[i]] = NodeStatus.Start;
            nodes.push(_nodes[i]);
        }

        // Initialize the address of admins
        for (uint j = 0; j < _admins.length; j++)
            admins[_admins[j]] = true;
    }

    function addAdmin(address _node) public onlyAdmin returns (bool) {
        admins[_node] = true;
        AddAdmin(_node, msg.sender);
        return true;
    }

    function newNode(address _node) public onlyClose(_node) returns (bool) {
        status[_node] = NodeStatus.Ready;
        NewNode(_node);
        return true;
    }

    function approveNode(address _node)
        public 
        onlyAdmin
        oneOperate
        onlyReady(_node)
        returns (bool) 
    {
        status[_node] = NodeStatus.Start;
        block_op[block.number] = true;
        nodes.push(_node);
        ApproveNode(_node);
        return true;
    }

    function deleteNode(address _node)
        public 
        onlyAdmin
        oneOperate
        onlyStart(_node)
        returns (bool)
    {
        var index = nodeIndex(_node);
        // Not found
        // @dev TODO: Make if a modifier
        if (index >= nodes.length)
            return false;

        status[_node] = NodeStatus.Close;
        // Remove the gap
        for (uint i = index; i < nodes.length - 1; i++)
            nodes[i] = nodes[i + 1];

        // Also delete the last element
        delete nodes[nodes.length - 1];
        nodes.length--;
        block_op[block.number] = false;
        DeleteNode(_node);
        return true;
    }

   /// Link address to a long string
    function concatNodes(address[] _add) internal returns (string nodeList) {
        if (_add.length > 0)
            nodeList = toString(_add[0]);

        for (uint i = 1; i < _add.length; i++)
            nodeList = nodeList.toSlice().concat(toString(_add[i]).toSlice());
    }

    /// Get the index in the nodes_of_start array
    function nodeIndex(address _node) internal returns (uint) {
        // Find the index of the member
        for (uint i = 0; i < nodes.length; i++) {
            if (_node == nodes[i])
                return i;
        }
        // If i == length, means not find
        return i;
    }

    /// Address to string
    /// The returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++)
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));

        return string(b);
    }

    function listNode() constant returns (string) {
        return concatNodes(nodes);
    }

    function getStatus(address _node) constant returns (uint8) {
        return uint8(status[_node]);
    }

    function isAdmin(address _node) constant returns (bool) {
        return admins[_node];
    }
}
