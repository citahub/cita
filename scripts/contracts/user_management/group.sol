pragma solidity ^0.4.18;

import "./group.sol";


/// @title Group contract
/// @notice Can not operate the parent. TBD
contract Group {
    
    address userManagementAddr = 0x00000000000000000000000000000000013241C2;

    bytes32 name;
    address parent;
    address[] accounts;
    address[] childs;

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
            if (!addressInArray(_accounts[i], accounts))
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
        for (uint i = 0; i < _accounts.length; i++)
            assert(addressDelete(_accounts[i], accounts));

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
        assert(addressDelete(_child, childs));
        ChildDeleted(_child);
    }

    /// @dev Add a child group
    function addChild(address _child)
        public
        onlyUserManagement
        returns (bool)
    {
        if (!addressInArray(_child, childs))
            childs.push(_child);

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

    /// @dev Check if the value in the array of address
    function addressInArray(address _value, address[] _array)
        private
        pure
        returns (bool)
    {
        // Have found the value in array
        for (uint i = 0; i < _array.length; i++) {
            if (_value == _array[i])
                return true;
        }
        // Not in
        return false;
    }

    /// @dev Delete the value of the address array
    function addressDelete(address _value, address[] storage _array)
        internal
        returns (bool)
    {
        var index = addressIndex(_value,  _array);
        // Not found
        if (index >= _array.length)
            return false;

        // Remove the gap
        for (uint i = index; i < _array.length - 1; i++) {
            _array[i] = _array[i + 1];
        }

        // Also delete the last element
        delete _array[_array.length - 1];
        _array.length--;
        return true;
    }

    /// @dev Get the index of the value in the bytes32 array
    /// @return The index. If i == length, means not find
    function addressIndex(address _value, address[] _array)
        private
        pure
        returns (uint i)
    {
        // Find the index of the value in the array
        for (i = 0; i < _array.length; i++) {
            if (_value == _array[i])
                return i;
        }
    }
}
