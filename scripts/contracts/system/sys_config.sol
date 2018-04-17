pragma solidity ^0.4.18;

interface SysConfigInterface {
    /// Get delay block number before validate
    function getDelayBlockNumber() public view returns (uint);

    /// Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() public view returns (bool);

    /// Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() public view returns (bool);

    /// The name of current chain
    function getChainName() public view returns (string);
    /// Update current chain name
    function setChainName(string) public;

    /// The id of current chain
    function getChainId() public view returns (uint32);

    /// The operator of current chain
    function getOperator() public view returns (string);
    /// Update current operator
    function setOperator(string) public;

    /// Current operator's website URL
    function getWebsite() public view returns (string);
    /// Update current operator's website URL
    function setWebsite(string) public;

    /// The interval time for creating a block (milliseconds)
    function getBlockInterval() public view returns (uint);
}

contract SysConfig is SysConfigInterface {

    uint delay_block_number;
    bool check_permission;
    bool check_quota;
    string chain_name;
    uint32 chain_id;
    string operator;
    string website;
    uint block_interval;

    /// Setup
    function SysConfig(
        uint _delay_block_number,
        bool _check_permission,
        bool _check_quota,
        string _chain_name,
        uint32 _chain_id,
        string _operator,
        string _website_url,
        uint _block_interval
    )
        public
    {
        delay_block_number = _delay_block_number;
        check_permission = _check_permission;
        check_quota = _check_quota;
        chain_name = _chain_name;
        chain_id = _chain_id;
        operator = _operator;
        website = _website_url;
        block_interval = _block_interval;
    }

    function getDelayBlockNumber() public view returns (uint) {
        return delay_block_number;
    }

    function getPermissionCheck() public view returns (bool) {
        return check_permission;
    }

    function getQuotaCheck() public view returns (bool) {
        return check_quota;
    }

    function getChainName() public view returns (string) {
        return chain_name;
    }
    function setChainName(string _chain_name) public {
        chain_name = _chain_name;
    }

    function getChainId() public view returns (uint32) {
        return chain_id;
    }

    function getOperator() public view returns (string) {
        return operator;
    }
    function setOperator(string _operator) public {
        operator = _operator;
    }

    function getWebsite() public view returns (string) {
        return website;
    }
    function setWebsite(string _website_url) public {
        website = _website_url;
    }

    function getBlockInterval() public view returns (uint) {
        return block_interval;
    }
}
