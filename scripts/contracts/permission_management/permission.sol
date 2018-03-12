pragma solidity ^0.4.18;


/// @title Permission contract
/// @notice Only be called by permission_management contract except query interface
///         TODO Add the modifier: Do not close the build-in permission
contract Permission {

    struct Resource {
        // Contract address
        address cont;
        // Function hash
        bytes4 func;
    }

    address permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    Resource[] resources;
    bytes32 name;

    event ResourcesAdded(address[] _conts, bytes4[] _funcs);
    event ResourcesDeleted(address[] _conts, bytes4[] _funcs);
    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _name);

    modifier onlyPermissionManagement {
        require(permissionManagementAddr == msg.sender);
        _;
    }

    /// @dev Constructor
    function Permission(bytes32 _name, address[] _conts, bytes4[] _funcs)
        public
    {
        name = _name;
        require(_addResources(_conts, _funcs));
    }

    /// @dev Add the resources
    function addResources(address[] _conts, bytes4[] _funcs)
        public
        onlyPermissionManagement
        returns (bool)
    {
        require(_addResources(_conts, _funcs));
        return true;
    }

    /// @dev Delete the resources
    function deleteResources(address[] _conts, bytes4[] _funcs)
        public
        onlyPermissionManagement
        returns (bool)
    {
        for (uint i = 0; i < _conts.length; i++)
            require(resourceDelete(_conts[i], _funcs[i]));

        ResourcesDeleted(_conts, _funcs);
        return true;
    }

    /// @dev Update permission's name
    function updateName(bytes32 _name)
        public
        onlyPermissionManagement
        returns (bool)
    {
        NameUpdated(name, _name);
        name = _name;
        return true;
    }

    /// @dev Destruct self
    function close()
        public
        onlyPermissionManagement
        returns (bool)
    {
        selfdestruct(msg.sender);
        return true;
    }

    /// @dev Check resource in the permission
    function inPermission(address cont, bytes4 func)
        public
        view
        returns (bool)
    {
        for (uint i = 0; i < resources.length; i++) {
            if (cont == resources[i].cont && func == resources[i].func)
                return true;
        }

        return false;
    }

    /// @dev Query the information of the permission
    function queryInfo()
        public
        view
        returns (bytes32, address[], bytes4[])
    {
        uint len = resources.length;
        address[] memory conts = new address[](len);
        bytes4[] memory funcs = new bytes4[](len);

        for (uint i = 0; i < resources.length; i++) {
            conts[i] = resources[i].cont;
            funcs[i] = resources[i].func;
        }

        return (name, conts, funcs);
    }

    /// @dev Query the name of the permission
    function queryName()
        public
        view
        returns (bytes32)
    {
        return name;
    }

    /// @dev Query the resource of the permission
    function queryResource()
        public
        view
        returns (address[], bytes4[])
    {
        uint len = resources.length;
        address[] memory conts = new address[](len);
        bytes4[] memory funcs = new bytes4[](len);

        for (uint i = 0; i < resources.length; i++) {
            conts[i] = resources[i].cont;
            funcs[i] = resources[i].func;
        }

        return (conts, funcs);
    }

    /// @dev Delete the value of the resources
    function resourceDelete(address _cont, bytes4 _func)
        private
        returns (bool)
    {
        var index = resourceIndex(_cont,  _func);
        // Not found
        if (index >= resources.length)
            return false;

        // Remove the gap
        // TODO Start from the bottom
        for (uint i = index; i < resources.length-1; i++)
            resources[i] = resources[i+1];

        // Also delete the last element
        delete resources[resources.length-1];
        resources.length--;
        return true;
    }

    /// @dev Get the index of the value in the resources
    /// @return The index. If i == length, means not find
    function resourceIndex(address _cont, bytes4 _func)
        private
        view
        returns (uint i)
    {
        for (i = 0; i < resources.length; i++) {
            if (_cont == resources[i].cont && _func == resources[i].func)
                return i;
        }
    }

    /// @dev Private: Add resources
    function _addResources(address[] _conts, bytes4[] _funcs)
        private
        returns (bool)
    {
        for (uint i = 0; i < _conts.length; i++) {
            if (!inResources(_conts[i], _funcs[i])) {
                Resource memory res = Resource(_conts[i], _funcs[i]);
                resources.push(res);
            }
        }

        ResourcesAdded(_conts, _funcs);
        return true;
    }

    /// @dev Check the duplicate resource
    function inResources(address _cont, bytes4 _func)
        private
        view
        returns (bool)
    {
        for (uint i = 0; i < resources.length; i++) {
            if (_cont == resources[i].cont && _func == resources[i].func)
                return true;
        }

        return false;
    }
}
