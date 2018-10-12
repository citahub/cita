pragma solidity ^0.4.24;

/// @title The interface of node_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface INodeManager {

    event ApproveNode(address indexed _node);
    event DeleteNode(address indexed _node);
    event SetStake(address indexed _node, uint _stake);

    /// @notice Approve to be consensus node. status will be start
    function approveNode(address _node) external returns (bool);

    /// @notice Delete the consensus node that has been approved. status will be close
    function deleteNode(address _node) external returns (bool);

    /// @notice List the consensus nodes that have been approved
    /// which means list the node whose status is start
    function listNode() external view returns (address[]);

    /// @notice Set node stake
    function setStake(address _node, uint64 stake) external;
    /*
     * @notice Get the status of the node:
     * @return 0: Close
     * @return 1: Start
     */
    function getStatus(address _node) external view returns (uint8);

    /// @notice Node stake list
    function listStake() external view returns (uint64[] _stakes);

    /// @notice Stake permillage
    function stakePermillage(address _node) external view returns (uint64);
}
