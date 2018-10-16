pragma solidity ^0.4.24;

/// @title The interface of role authorization
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IRoleAuth {
    /// @notice only role management
    function setRole(address _account, address _role) external returns (bool);

    /// @notice only role management
    function cancelRole(address _account, address _role) external returns (bool);

    /// @notice only role management
    function clearAuthOfRole(address _role) external returns (bool);

    /// @notice only role management
    function setPermissionsOfRole(address _role, address[] _permissions) external returns (bool);

    /// @notice only role management
    function cancelPermissionsOfRole(address _role, address[] _permissions) external returns (bool);

    /// @notice only role management
    function clearRole(address _account) external returns (bool);

    function queryRoles(address _account) external returns (address[]);

    function queryAccounts(address _role) external returns (address[]);
}
