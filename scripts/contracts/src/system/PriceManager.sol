pragma solidity ^0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrPublic.sol";
import "../interfaces/IPriceManager.sol";

/// @title Quota Price Manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract PriceManager is IPriceManager, ReservedAddrPublic {
    uint quotaPrice = 1;

    Admin admin = Admin(adminAddr);

    event SetQuotaPrice(uint indexed _quotaPrice);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    /// @notice Setup
    constructor(uint _quotaPrice)
        public
    {
        quotaPrice = _quotaPrice;
    }

    /// @notice Set quota price
    function setQuotaPrice(uint _quotaPrice)
        external
        onlyAdmin
        returns (bool)
    {
        require(_quotaPrice > 0, "The quota price should larger than zero.");
        quotaPrice = _quotaPrice;
        emit SetQuotaPrice(_quotaPrice);
        return true;
    }

    /// @notice Get guota price
    function getQuotaPrice()
        public
        view
        returns (uint)
    {
        return quotaPrice;
    }
}
