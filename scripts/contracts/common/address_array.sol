pragma solidity ^0.4.18;


library AddressArray {

    /// @dev Remove the value of the address array
    function remove(address _value, address[] storage _array)
        internal
        returns (bool)
    {
        uint _index = index(_value,  _array);
        // Not found
        if (_index >= _array.length)
            return false;

        // Move the last element to the index of array
        _array[_index] = _array[_array.length - 1];

        // Also delete the last element
        delete _array[_array.length - 1];
        _array.length--;
        return true;
    }

    /// @dev Get the index of the value in the bytes32 array
    /// @return The index. If i == length, means not find
    function index(address _value, address[] _array)
        pure
        internal
        returns (uint i)
    {
        // Find the index of the value in the array
        for (i = 0; i < _array.length; i++) {
            if (_value == _array[i])
                return i;
        }
    }

    /// @dev Check if the value in the array of address
    function exist(address _value, address[] _array)
        pure
        internal
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

    /// @dev Check the array of address is nul
    function isNull(address[] _array)
        pure
        internal
        returns (bool)
    {
        if (_array.length == 0)
            return true;
        for (uint i = 0; i < _array.length; i++) {
            if (address(0x0) != _array[i])
                return false;
        }

        return true;
    }
}
