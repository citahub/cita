pragma solidity ^0.4.18;

import "./node_interface.sol";
import "./address_array.sol";

contract NodeManager is NodeInterface {

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
    function NodeManager(address[] _nodes, address[] _admins) public {
        // Initialize the address to Start
        for (uint i = 0; i < _nodes.length; i++) {
            status[_nodes[i]] = NodeStatus.Start;
            nodes.push(_nodes[i]);
        }

        // Initialize the address of admins
        for (uint j = 0; j < _admins.length; j++)
            admins[_admins[j]] = true;
    }

    function addAdmin(address _node)
        public
        onlyAdmin
        returns (bool)
    {
        admins[_node] = true;
        AddAdmin(_node, msg.sender);
        return true;
    }

    function newNode(address _node)
        public
        onlyClose(_node)
        returns (bool)
    {
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
        require(AddressArray.remove(_node, nodes));
        block_op[block.number] = false;
        status[_node] = NodeStatus.Close;
        DeleteNode(_node);
        return true;
    }

    function listNode() view public returns (address[]) {
        return nodes;
    }

    function getStatus(address _node) view public returns (uint8) {
        return uint8(status[_node]);
    }

    function isAdmin(address _node) view public returns (bool) {
        return admins[_node];
    }
}
