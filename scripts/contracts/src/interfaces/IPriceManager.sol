pragma solidity ^0.4.24;

/// @title The interface of quota price manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IPriceManager {
    function setQuotaPrice(uint _quotaPrice) external returns (bool);

    function getQuotaPrice() external returns (uint);

}
