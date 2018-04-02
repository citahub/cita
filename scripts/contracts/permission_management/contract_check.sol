pragma solidity ^0.4.18;


library ContractCheck {

    /// @dev Check an address is contract address
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
