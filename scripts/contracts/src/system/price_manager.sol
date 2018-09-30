
pragma solidity ^0.4.24;

import "../common/admin.sol";
import "../common/address.sol";


/// @title Quota Price Manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract PriceManager is ReservedAddress{
    uint quotaPrice = 1;

    Admin admin = Admin(adminAddr);

    event SetQuotaPrice(uint indexed _quotaPrice);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
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