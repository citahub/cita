pragma solidity ^0.4.24;


contract OpUint8 {

    uint8 public u8;
    uint8[] public u8array;
    mapping(uint8 => string) public u8map;

    event IndexU8(uint8 indexed x);

    function set(uint8 x) public {
        u8 = x;
        u8array.push(x);
        u8map[x] = 'test';
        emit IndexU8(x);
    }

    function list() public view returns (uint8[]) {
        return u8array;
    }
}
