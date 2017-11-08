pragma solidity ^0.4.14;

interface NodeInterface {

    event NewNode(address indexed _node);
    event ApproveNode(address indexed _node);
    event DeleteNode(address indexed _node);
    event AddAdmin(address indexed _node, address indexed _sender);

    function addAdmin(address) public returns (bool);
    /// @dev Apply to be consensus node. status will be ready
    function newNode(address _node) public returns (bool);
    /// @dev Approve to be consensus node. status will be start
    function approveNode(address _node) public returns (bool);
    /// @dev Delete the consensus node that has been approved. status will be close
    function deleteNode(address _node) public returns (bool);
    /// @dev List the consensus nodes that have been approved
    ///      which means list the node whose status is start
    function listNode() constant returns (string);
    /*
     * @dev Get the status of the node:
     * @return 0: Close
     * @return 1: Ready
     * @return 2: Start
     */
    function getStatus(address _node) constant returns (uint8);
    function isAdmin(address) constant returns (bool);
}
