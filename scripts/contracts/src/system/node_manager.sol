pragma solidity ^0.4.24;

import "../lib/safe_math.sol";
import "../common/error.sol";
import "../common/admin.sol";
import "../permission_management/authorization.sol";


/// @title The interface of node_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface NodeInterface {

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


/// @title Node manager contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020001
contract NodeManager is NodeInterface, Error, ReservedAddress, EconomicalType {

    mapping(address => NodeStatus) public status;
    // Recode the operation of the block
    mapping(uint => bool) block_op;
    // Consensus node list
    address[] nodes;
    mapping(address => uint64) stakes;

    // Default: Close
    enum NodeStatus { Close, Start }

    Admin admin = Admin(adminAddr);
    Authorization auth = Authorization(authorizationAddr);
    SysConfig sysConfig = SysConfig(sysConfigAddr);

    // Should operate one time in a block
    modifier oneOperate {
        if (!block_op[block.number])
            _;
        else {
            emit ErrorLog(ErrorType.NotOneOperate, "should operate one time in a block");
            return;
        }
    }

    modifier onlyClose(address _node) {
        if (NodeStatus.Close == status[_node])
            _;
        else {
            emit ErrorLog(ErrorType.NotClose, "node does not close");
            return;
        }
    }

    modifier onlyStart(address _node) {
        if (NodeStatus.Start == status[_node])
            _;
        else {
            emit ErrorLog(ErrorType.NotStart, "node does not start");
            return;
        }
    }

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    modifier checkPermission(address _permission) {
        require(auth.checkPermission(msg.sender, _permission), "permission denied.");
        _;
    }

    modifier OnlyChargeModel() {
        if(sysConfig.getEconomicalModel() == EconomicalModel.Charge) 
            _;
        else {
            return;
        }
    }

    /// @notice Setup
    constructor(address[] _nodes, uint64[] _stakes)
        public
    {
        // Initialize the address to Start
        require(_nodes.length == _stakes.length, "nodes's length not equal to stakes's length.");
        for (uint i = 0; i < _nodes.length; i++) {
            status[_nodes[i]] = NodeStatus.Start;
            nodes.push(_nodes[i]);
            stakes[_nodes[i]] = _stakes[i];
        }
    }

    /// @notice Set node stake
    function setStake(address _node, uint64 stake)
        public
        onlyAdmin
        checkPermission(builtInPermissions[17])
        returns (bool)
    {
        require(AddressArray.exist(_node, nodes), "node not exist.");
        emit SetStake(_node, stake);
        stakes[_node] = stake;
        return true;
    }

    /// @notice Approve the new node
    /// @param _node The node to be approved
    /// @return true if successed, otherwise false
    function approveNode(address _node)
        public
        onlyAdmin
        oneOperate
        onlyClose(_node)
        checkPermission(builtInPermissions[15])
        returns (bool)
    {
        status[_node] = NodeStatus.Start;
        block_op[block.number] = true;
        nodes.push(_node);
        emit ApproveNode(_node);
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
        checkPermission(builtInPermissions[16])
        returns (bool)
    {
        require(AddressArray.remove(_node, nodes), "remove node failed.");
        block_op[block.number] = false;
        status[_node] = NodeStatus.Close;
        stakes[_node] = 0;
        emit DeleteNode(_node);
        return true;
    }

    /// @notice Query the consensus nodes
    /// @return All the consensus nodes
    function listNode()
        public
        view
        returns (address[])
    {
        return nodes;
    }

    /// @notice Query the status of node
    /// @param _node The node to be deleted
    /// @return The status of the node
    function getStatus(address _node)
        public
        view
        returns (uint8)
    {
        return uint8(status[_node]);
    }

    /// @notice Node stake list
    /// @return All the node stake list
    function listStake()
        public
        view
        returns (uint64[] _stakes)
    {
        _stakes = new uint64[](nodes.length);
        for (uint j = 0; j < nodes.length; j++) {
            _stakes[j] = stakes[nodes[j]];
        }
        return _stakes;
    }

    /// @notice Stake permillage
    /// This is the slot number which ignore the remainder, not exactly precise.
    /// https://en.wikipedia.org/wiki/Largest_remainder_method
    /// Hare quota
    function stakePermillage(address _node) 
        public 
        view 
        OnlyChargeModel
        returns (uint64) 
    {
        uint total;
        for (uint j = 0; j < nodes.length; j++) {
            total = SafeMath.add(uint(total), uint(stakes[nodes[j]]));
        }

        if(total == 0) {
            return; 
        }
        return uint64(SafeMath.div(SafeMath.mul(stakes[_node], 1000), total));
    }
}
