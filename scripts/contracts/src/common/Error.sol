pragma solidity 0.4.24;

/// @title The enum data of error
/// @author ["Rivtower Technologies <contact@rivtower.com>"]
contract Error {

    enum ErrorType {
        NotAdmin,
        OutOfBaseLimit,
        OutOfBlockLimit,
        NoParentChain,
        NoSideChain,
        NotOneOperate,
        NotClose,
        NotStart,
        NotReady
    }

    event ErrorLog(ErrorType indexed errorType, string msg);
}
