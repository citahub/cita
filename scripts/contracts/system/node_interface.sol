pragma solidity ^0.4.18;


/// @title The interface of node_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface NodeInterface {

    event NewNode(address indexed _node);
    event ApproveNode(address indexed _node);
    event DeleteNode(address indexed _node);
    event AddAdmin(address indexed _account, address indexed _sender);

    /// @notice Add an admin
    function addAdmin(address) public returns (bool);
    /// @notice Apply to be consensus node. status will be ready
    function newNode(address _node) public returns (bool);
    /// @notice Approve to be consensus node. status will be start
    function approveNode(address _node) public returns (bool);
    /// @notice Delete the consensus node that has been approved. status will be close
    function deleteNode(address _node) public returns (bool);
    /// @notice List the consensus nodes that have been approved
    ///      which means list the node whose status is start
    function listNode() view public returns (address[]);
    /*
     * @notice Get the status of the node:
     * @return 0: Close
     * @return 1: Ready
     * @return 2: Start
     */
    function getStatus(address _node) view public returns (uint8);
    /// @notice Check the account is admin
    function isAdmin(address) view public returns (bool);
}
