pragma solidity ^0.4.18;

/// @notice TODO: Address and bytes32. refactor the duplicate code.
///               The elements of set is different each other.
library Util {

    enum SetOp { None, Union, Interaction, Diff }

    function setOpBytes32(bytes32[] _one, bytes32[] _other, SetOp _op)
        pure
        internal
        returns (bytes32[])
    {
        if (SetOp.Union == _op)
            return opUnionBytes32(_one, _other);
        else if (SetOp.Interaction == _op)
            return opInteractionBytes32(_one, _other);
        else if (SetOp.Diff == _op)
            return opDiffBytes32(_one, _other);
    }

    /// @dev Union set of bytes32
    function opUnionBytes32(bytes32[] _one, bytes32[] _other)
        pure
        internal
        returns (bytes32[] result)
    {
        uint index = _other.length;
        bool flag;

        for (uint i = 0; i < _other.length; i++)
            result[i] = _other[i];

        for (uint j = 0; j < _one.length; j++) {
            flag = true;

            for (uint k = 0; k < _other.length; k++) {
                if (_one[j] == _other[k])
                    flag = false;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @dev Interaction set of bytes32
    function opInteractionBytes32(bytes32[] _one, bytes32[] _other)
        pure
        internal
        returns (bytes32[] result)
    {
        uint index;
        bool flag;

        for (uint i = 0; i < _one.length; i++) {
            flag = false;

            for (uint j = 0; j < _other.length; j++) {
                if (_one[i] == _other[j])
                    flag = true;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @dev Diff set of bytes32
    function opDiffBytes32(bytes32[] _one, bytes32[] _other)
        pure
        internal
        returns (bytes32[] result)
    {
        uint index;
        bool flag;

        for (uint i = 0; i < _one.length; i++) {
            flag = true;

            for (uint j = 0; j < _other.length; j++) {
                if (_one[i] == _other[j])
                    flag = false;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @notice Set operation of address
    function setOpAddress(address[] _one, address[] _other, SetOp _op)
        pure
        internal
        returns (address[])
    {
        if (SetOp.Union == _op)
            return opUnionAddress(_one, _other);
        else if (SetOp.Interaction == _op)
            return opInteractionAddress(_one, _other);
        else if (SetOp.Diff == _op)
            return opDiffAddress(_one, _other);
    }

    /// @dev Union set of address
    function opUnionAddress(address[] _one, address[] _other)
        pure
        internal
        returns (address[] result)
    {
        uint index = _other.length;
        bool flag;

        for (uint i = 0; i < _other.length; i++)
            result[i] = _other[i];

        for (uint j = 0; j < _one.length; j++) {
            flag = true;

            for (uint k = 0; k < _other.length; k++) {
                if (_one[j] == _other[k])
                    flag = false;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @dev Interaction set of address
    function opInteractionAddress(address[] _one, address[] _other)
        pure
        internal
        returns (address[] result)
    {
        uint index;
        bool flag;

        for (uint i = 0; i < _one.length; i++) {
            flag = false;

            for (uint j = 0; j < _other.length; j++) {
                if (_one[i] == _other[j])
                    flag = true;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @dev Diff set of address
    function opDiffAddress(address[] _one, address[] _other)
        pure
        internal
        returns (address[] result)
    {
        uint index;
        bool flag;

        for (uint i = 0; i < _one.length; i++) {
            flag = true;

            for (uint j = 0; j < _other.length; j++) {
                if (_one[i] == _other[j])
                    flag = false;
            }

            if (flag) {
                result[index] = _one[i];
                index++;
            }
        }
    }

    /// @dev Delete the value of the bytes32 array
    function bytes32Delete(bytes32 _value, bytes32[] storage _array)
        internal
        returns (bool)
    {
        var index = bytes32Index(_value,  _array);
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
    function bytes32Index(bytes32 _value, bytes32[] _array)
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

    /// @dev Check if the value in the array of bytes32
    function bytes32InArray(bytes32 _value, bytes32[] _array)
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

    /// @dev Check if the values in the array of bytes32
    /// @notice TODO: Check SubSet
    function bytes32SubSet(bytes32[] _subSet, bytes32[] _array)
        pure
        internal
        returns (bool)
    {
        for (uint i = 0; i < _subSet.length; i++) {
            if (bytes32InArray(_subSet[i], _array))
                continue;
            else
                return false;
        }

        return true;
    }

    /// @dev Check if the value in the array of address
    function addressInArray(address _value, address[] _array)
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

    /// @dev Replace the value in the array of bytes32
    function bytes32Replace(bytes32 _old, bytes32 _new, bytes32[] storage _array)
        internal
        returns(bool)
    {
        // Find the value in array and repalce it
        for (uint i = 0; i < _array.length; i++) {
            if (_old == _array[i]) {
                _array[i]  = _new;
                return true;
            }
        }
    }

    /// @dev Check the array of bytes32 is nul
    function bytes32ArrayNul(bytes32[] _array)
        pure
        internal
        returns (bool)
    {
        if (_array.length == 0)
            return true;
        for (uint i = 0; i < _array.length; i++) {
            if (bytes32(0x0) != _array[i])
                return false;
        }

        return true;
    }

    /// @dev Check the array of address is nul
    function addressArrayNul(address[] _array)
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
