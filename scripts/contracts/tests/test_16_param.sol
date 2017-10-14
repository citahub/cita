pragma solidity ^0.4.14;

contract FunctionParam16 {
    uint[] params;

    event Set(uint x0);

    function set(uint x0, uint x1, uint x2, uint x3, uint x4, uint x5, uint x6, uint x7, 
                 uint x8, uint x9, uint x10, uint x11, uint x12, uint x13, uint x14, uint x15) {
        params.push(x0);
        params.push(x1);
        params.push(x2);
        params.push(x3);
        params.push(x4);
        params.push(x5);
        params.push(x6);
        params.push(x7);
        params.push(x8);
        params.push(x9);
        params.push(x10);
        params.push(x11);
        params.push(x12);
        params.push(x13);
        params.push(x14);
        params.push(x15);
        
        Set(x0);
    }

    function get() constant returns (uint x0, uint x1, uint x2, uint x3, uint x4, uint x5, uint x6, uint x7, uint x8, uint x9, uint x10, uint x11, uint x12, uint x13, uint x14, uint x15) {
        return (params[0], x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, params[15]);
    }
}
