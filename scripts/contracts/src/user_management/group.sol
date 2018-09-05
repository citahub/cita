pragma solidity ^0.4.24;

import "../lib/address_array.sol";
import "../common/address.sol";


/// @title Group contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: Created by permissionCreator
///         The interface can be called: Only query type
contract Group is ReservedAddress {

    bytes32 name;
    address parent;
    address[] accounts;
    address[] children;

    event GroupNewed(address indexed _parent, bytes32 indexed _name, address[] _accounts);
    event AccountsAdded(address[] _accounts);
    event AccountsDeleted(address[] _accounts);
    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event ChildDeleted(address indexed _child);
    event ChildAdded(address indexed _child);

    modifier onlyUserManagement {
        require(userManagementAddr == msg.sender, "permission denied.");
        _;
    }

    /// @notice Constructor
    constructor(address _parent, bytes32 _name, address[] _accounts)
        public
    {
        parent = _parent;
        name = _name;
        accounts = _accounts;
        emit GroupNewed(_parent, _name, _accounts);
    }

    /// @notice Add accounts
    /// @param _accounts The accounts to be added
    /// @return True if successed, otherwise false
    function addAccounts(address[] _accounts)
        public
        onlyUserManagement
        returns (bool)
    {
        for (uint i = 0; i<_accounts.length; i++) {
            if (!AddressArray.exist(_accounts[i], accounts))
                accounts.push(_accounts[i]);
        }

        emit AccountsAdded(_accounts);
        return true;
    }

    /// @notice Delete accounts
    /// @param _accounts The accounts to be deleted
    /// @return True if successed, otherwise false
    function deleteAccounts(address[] _accounts)
        public
        onlyUserManagement
        returns (bool)
    {
        require(_accounts.length < accounts.length, "deleteAccounts failed.");

        for (uint i = 0; i < _accounts.length; i++)
            assert(AddressArray.remove(_accounts[i], accounts));

        emit AccountsDeleted(_accounts);
        return true;
    }

    /// @notice Update group name
    /// @param _name  The new name to be updated
    /// @return True if successed, otherwise false
    function updateName(bytes32 _name)
        public
        onlyUserManagement
        returns (bool)
    {
        emit NameUpdated(name, _name);
        name = _name;
        return true;
    }

    /// @notice Delete a child group
    /// @param _child The child group to be deleted
    /// @return True if successed, otherwise false
    function deleteChild(address _child)
        public
        onlyUserManagement
        returns (bool)
    {
        assert(AddressArray.remove(_child, children));
        emit ChildDeleted(_child);
        return true;
    }

    /// @notice Add a child group
    /// @param _child The child group to be added
    /// @return True if successed, otherwise false
    function addChild(address _child)
        public
        onlyUserManagement
        returns (bool)
    {
        if (!AddressArray.exist(_child, children))
            children.push(_child);

        emit ChildAdded(_child);
        return true;
    }

    /// @notice Destruct self
    /// @return True if successed, otherwise false
    function close()
        public
        onlyUserManagement
    {
        selfdestruct(msg.sender);
    }

    /// @notice Query the information of the group
    /// @dev TODO Include the children group
    /// @return Name and accounts of group
    function queryInfo()
        public
        view
        returns (bytes32, address[])
    {
        return (name, accounts);
    }

    /// @notice Query the name of the group
    /// @return The name of group
    function queryName()
        public
        view
        returns (bytes32)
    {
        return name;
    }

    /// @notice Query the accounts of the group
    /// @return The accounts of group
    function queryAccounts()
        public
        view
        returns (address[])
    {
        return accounts;
    }

    /// @notice Query the child of the group
    /// @dev TODO Rename queryChildren
    /// @return The children of group
    function queryChild()
        public
        view
        returns (address[])
    {
        return children;
    }

    /// @notice Query the length of children of the group
    /// @return The number of the children group
    function queryChildLength()
        public
        view
        returns (uint)
    {
        return children.length;
    }

    /// @notice Query the parent of the group
    /// @return The parent of the group
    function queryParent()
        public
        view
        returns (address)
    {
        return parent;
    }

    /// @notice Check the account in the group
    /// @return Ture if the account in the group, otherwise false
    function inGroup(address _account)
        public
        view
        returns (bool)
    {
        return AddressArray.exist(_account, accounts);
    }
}
