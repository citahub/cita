pragma solidity ^0.4.11;

interface NodeInterface {
    // apply to be consensus node. status will be ready
    function newNode(address _node) returns (bool);
    // approve to be consensus node. status will be start
    function approveNode(address _node) returns (bool);
    // delete the consensus node that has been approved. status will be close 
    function deleteNode(address _node) returns (bool);
    // list the consensus nodes that have been approved
    // which means list the node whose status is start
    function listNode() constant returns (string);
    // get the status of the node
    // 0: close;
    // 1: ready;
    // 2: start
    function getStatus(address _node) constant returns (uint8);
}
