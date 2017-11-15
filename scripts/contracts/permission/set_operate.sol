pragma solidity ^0.4.14;

/// @notice TODO: Address and bytes32. refactor the duplicate code.
///               The elements of set is different each othet. 
library SetOperate {

    enum SetOp { None, Union, Interaction, Diff }

    function setOpBytes32(bytes32[] _one, bytes32[] _other, SetOp _op) internal returns (bytes32[]) {
        if (SetOp.Union == _op)
            return opUnionBytes32(_one, _other);
        else if (SetOp.Interaction == _op)
            return opInteractionBytes32(_one, _other);
        else if (SetOp.Diff == _op)
            return opDiffBytes32(_one, _other);
    }

    /// @dev Union set of bytes32
    function opUnionBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        bytes32[] memory result;
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

        return result;
    }

    /// @dev Interaction set of bytes32
    function opInteractionBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        bytes32[] memory result;
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

        return result;
    }

    /// @dev Diff set of bytes32
    function opDiffBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        bytes32[] memory result;
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

        return result;
    }

    /// @notice Set operation of address
    function setOpAddress(address[] _one, address[] _other, SetOp _op) internal returns (address[]) {
        if (SetOp.Union == _op)
            return opUnionAddress(_one, _other);
        else if (SetOp.Interaction == _op)
            return opInteractionAddress(_one, _other);
        else if (SetOp.Diff == _op)
            return opDiffAddress(_one, _other);
    }

    /// @dev Union set of address
    function opUnionAddress(address[] _one, address[] _other) internal returns (address[]) {
        address[] memory result;
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

        return result;
    }

    /// @dev Interaction set of address
    function opInteractionAddress(address[] _one, address[] _other) internal returns (address[]) {
        address[] memory result;
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

        return result;
    }

    /// @dev Diff set of address
    function opDiffAddress(address[] _one, address[] _other) internal returns (address[]) {
        address[] memory result;
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

        return result;
    }
}
