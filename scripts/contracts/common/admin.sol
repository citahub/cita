pragma solidity ^0.4.24;

import "./error.sol";


/// @title A common contract about admin
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract Admin is Error {

    address public admin;

    event AdminUpdated(address indexed _account, address indexed _old, address indexed _sender);

    modifier onlyAdmin {
        if (isAdmin(msg.sender))
            _;
        else return;
    }

    constructor(address _account) public {
        admin = _account;
    }

    /// @notice Update the admin
    ///         Be careful to use it. TODO update the permissions of admin
    /// @param _account Address of the admin
    /// @return true if successed, otherwise false
    function update(address _account)
        external
        onlyAdmin
        returns (bool)
    {
        emit AdminUpdated(_account, admin, msg.sender);
        admin = _account;
        return true;
    }

    /// @notice Check the account is admin
    /// @param _account The address to be checked
    /// @return true if it is, otherwise false
    function isAdmin(address _account)
        public
        returns (bool)
    {
        if (_account == admin) {
            return true; 
        } 
        emit ErrorLog(ErrorType.NotAdmin, "Not the admin account");
    }
}
