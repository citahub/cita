pragma solidity ^0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrConstant.sol";
import "../interfaces/IAutoExec.sol";
import "../lib/ContractCheck.sol";

/// @title Manage the scheduled executing contract.
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract AutoExec is IAutoExec, ReservedAddrConstant {

    address public contAddr;

    event Registered(address indexed _contAddr);

    modifier onlyAdmin {
        Admin admin = Admin(adminAddr);
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    /// @notice Register an autoExec contract
    ///  There's only one. You can call it to replace another
    function register(address _contAddr)
        external
        onlyAdmin
    {
        require(
            ContractCheck.isContract(_contAddr),
            "address should be a contract"
        );
        emit Registered(_contAddr);
        contAddr = _contAddr;
    }

    /// @notice auto exec
    function autoExec()
        external
    {
        require(msg.sender == 0x0, "only be called by executor");
        // In case the contract selfdestruct
        require(
            ContractCheck.isContract(contAddr),
            "address should be a contract"
        );
        IAutoExec(contAddr).autoExec();
    }
}
