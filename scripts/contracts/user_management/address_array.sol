pragma solidity ^0.4.18;


/// @title A library for operation of address array
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice
/// > The --allow-paths command line option for solc only works with absolute paths. It would be useful if it could be used with relative paths such as ../ and the current working directory(.).
///
/// Mode details at [issue](https://github.com/ethereum/solidity/issues/2928)
///
/// So using hard link for now. e.g.
///
/// ```
/// $ pwd
///
/// .../cita/scripts/contracts/permission_management
///
/// ```
///
/// Use `ln` command:
///
/// ```
/// ln ../common/address_array.sol ./ -f
/// ```
/// @dev TODO more interface
library AddressArray {

    /// @notice Remove the value of the address array
    /// @param _value The value of address to be removed
    /// @param _array The array to remove from
    /// @return true if successed, false otherwise
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

    /// @notice Get the index of the value in the array
    /// @param _value The value of address to be founded
    /// @param _array The array to find from
    /// @return The index if founded, length of array otherwise
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

    /// @notice Check if the value in the array
    /// @param _value The value of address to be checked
    /// @param _array The array to check from
    /// @return true if existed, false otherwise
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

    /// @notice Check the array of address is null:
    /// 1. the length is zero 2. all values of array are zero
    /// @param _array The array to check from
    /// @return true if is null, false otherwise
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
