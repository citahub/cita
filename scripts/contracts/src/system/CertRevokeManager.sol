pragma solidity 0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrPublic.sol";

contract CertRevokeManager is ReservedAddrPublic {

    // certificate revoked list, maintains as node address.
    address[] revokedList;

    Admin admin = Admin(adminAddr);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    function revoke(address cert_addr)
        public
        onlyAdmin
    {
        // Do not push a repeated address into the revokedList.
        for (uint i = 0; i < revokedList.length; i++) {
            if (revokedList[i] == cert_addr) {
                return;
            }
        }
        revokedList.push(cert_addr);
    }

    function getCrl()
        public
        view
        returns (address[])
    {
        return revokedList;
    }
}