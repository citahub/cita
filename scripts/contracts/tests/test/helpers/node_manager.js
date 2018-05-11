/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr, admin } = config.contract.node_manager;

const node = web3.eth.contract(abi);
const nodeContractIns = node.at(addr);

const genTxParams = util.genTxParams;

// addAdmin
const addAdmin = function (account, _sender = sender) {
    return nodeContractIns.addAdmin.sendTransaction(
                account,
                genTxParams(_sender)
            );
};

// newNode
const newNode = function (node, _sender = sender) {
    return nodeContractIns.newNode.sendTransaction(
                node,
                genTxParams(_sender)
            );
};

// approveNode
const approveNode = function (node, _sender = sender) {
    return nodeContractIns.approveNode.sendTransaction(
                node,
                genTxParams(_sender)
            );
};

// deleteNode
const deleteNode = function (node, _sender = sender) {
    return nodeContractIns.deleteNode.sendTransaction(
                node,
                genTxParams(_sender)
            );
};

// listNode
const listNode = function () {
    return nodeContractIns.listNode.call();
};

// getStatus
const getStatus = function (node) {
    return nodeContractIns.getStatus.call(node);
};

// isAdmin
const isAdmin = function (account) {
    return nodeContractIns.isAdmin.call(account);
};

module.exports = {
    addAdmin,
    newNode,
    approveNode,
    deleteNode,
    listNode,
    getStatus,
    isAdmin
};
