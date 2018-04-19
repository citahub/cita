pragma solidity ^0.4.18;


/// @title A library for operation of contract
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
library ContractCheck {

    /// @notice Check an address is contract address
    /// @param _target The address to be checked
    /// @return true if successed, false otherwise
    function isContract(address _target)
        internal
        view
        returns (bool)
    {
        uint size;
        assembly { size := extcodesize(_target) }
        return size > 0;
    }
}
