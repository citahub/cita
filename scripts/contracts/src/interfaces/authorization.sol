pragma solidity ^0.4.24;

/// @title The interface of authorization
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IAuthorization {
    /// @notice only permission management
    function setAuth(address _account, address _permission) external returns (bool);

    /// @notice only permission management
    function cancelAuth(address _account, address _permission) external returns (bool);

    /// @notice only permission management
    function clearAuth(address _account) external returns (bool);

    /// @notice only permission management
    function clearAuthOfPermission(address _permission) external returns (bool);

    function queryPermissions(address _account) external returns (address[]);

    function queryAccounts(address _permission) external returns (address[]);

    function queryAllAccounts() external returns (address[]);

    /// @notice Check account has a resource(deprecation)
    function checkResource(address, address, bytes4) external returns (bool);

    function checkPermission(address _account, address _permission) external returns (bool);
}
