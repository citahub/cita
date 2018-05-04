pragma solidity ^0.4.18;

import "./node_interface.sol";
import "../common/address_array.sol";


/// @title Node manager contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0x00000000000000000000000000000000013241a2
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

    /// @notice Setup
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

    /// @notice Add an admin
    /// @param _account Address of the admin
    /// @return true if successed, otherwise false
    function addAdmin(address _account)
        public
        onlyAdmin
        returns (bool)
    {
        admins[_account] = true;
        AddAdmin(_account, msg.sender);
        return true;
    }

    /// @notice Add a new node
    /// @param _node The node to be added
    /// @return true if successed, otherwise false
    function newNode(address _node)
        public
        onlyClose(_node)
        returns (bool)
    {
        status[_node] = NodeStatus.Ready;
        NewNode(_node);
        return true;
    }

    /// @notice Approve the new node
    /// @param _node The node to be approved
    /// @return true if successed, otherwise false
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

    /// @notice Delete the node
    /// @param _node The node to be deleted
    /// @return true if successed, otherwise false
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

    /// @notice Query the consensus nodes
    /// @return All the consensus nodes
    function listNode() view public returns (address[]) {
        return nodes;
    }

    /// @notice Query the status of node
    /// @param _node The node to be deleted
    /// @return The status of the node
    function getStatus(address _node) view public returns (uint8) {
        return uint8(status[_node]);
    }

    /// @notice Check the account is admin
    /// @param _account The address to be checked
    /// @return true if it is, otherwise false
    function isAdmin(address _account) view public returns (bool) {
        return admins[_account];
    }
}
