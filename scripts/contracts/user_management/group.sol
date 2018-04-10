pragma solidity ^0.4.18;

import "./group.sol";
import "./address_array.sol";


/// @title Group contract
/// @notice Can not operate the parent. TBD
contract Group {

    address userManagementAddr = 0x00000000000000000000000000000000013241C2;

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
        require(userManagementAddr == msg.sender);
        _;
    }

    /// @dev Constructor
    function Group(address _parent, bytes32 _name, address[] _accounts)
        public
    {
        parent = _parent;
        name = _name;
        accounts = _accounts;
        GroupNewed(_parent, _name, _accounts);
    }

    /// @dev Add accounts
    function addAccounts(address[] _accounts)
        public
        onlyUserManagement
        returns (bool)
    {
        for (uint i = 0; i<_accounts.length; i++) {
            if (!AddressArray.exist(_accounts[i], accounts))
                accounts.push(_accounts[i]);
        }

        AccountsAdded(_accounts);
        return true;
    }

    /// @dev Delete accounts
    function deleteAccounts(address[] _accounts)
        public
        onlyUserManagement
        returns (bool)
    {
        require(_accounts.length < accounts.length);

        for (uint i = 0; i < _accounts.length; i++)
            assert(AddressArray.remove(_accounts[i], accounts));

        AccountsDeleted(_accounts);
        return true;
    }

    /// @dev Update group name
    function updateName(bytes32 _name)
        public
        onlyUserManagement
        returns (bool)
    {
        NameUpdated(name, _name);
        name = _name;
        return true;
    }

    /// @dev Delete a child group
    function deleteChild(address _child)
        public
        onlyUserManagement
        returns (bool)
    {
        assert(AddressArray.remove(_child, children));
        ChildDeleted(_child);
        return true;
    }

    /// @dev Add a child group
    function addChild(address _child)
        public
        onlyUserManagement
        returns (bool)
    {
        if (!AddressArray.exist(_child, children))
            children.push(_child);

        ChildAdded(_child);
        return true;
    }

    /// @dev Destruct self
    function close()
        public
        onlyUserManagement
        returns (bool)
    {
        selfdestruct(msg.sender);
        return true;
    }

    /// @dev Query the information of the group
    function queryInfo()
        public
        view
        returns (bytes32, address[])
    {
        return (name, accounts);
    }

    /// @dev Query the name of the group
    function queryName()
        public
        view
        returns (bytes32)
    {
        return name;
    }

    /// @dev Query the accounts of the group
    function queryAccounts()
        public
        view
        returns (address[])
    {
        return accounts;
    }

    /// @dev Query the child of the group
    function queryChild()
        public
        view
        returns (address[])
    {
        return children;
    }

    /// @dev Query the length of children of the group
    function queryChildLength()
        public
        view
        returns (uint)
    {
        return children.length;
    }

    /// @dev Query the parent of the group
    function queryParent()
        public
        view
        returns (address)
    {
        return parent;
    }

    /// @dev Check the account in the group
    function inGroup(address _account)
        public
        view
        returns (bool)
    {
        return AddressArray.exist(_account, accounts);
    }
}
