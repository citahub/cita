contract ParaType {
    uint8 _u8;
    uint16 _u16;
    uint32 _u32;
    uint256 _u256;
    uint _u;

    int8 _i8;
    int16 _i16;
    int32 _i32;
    int256 _i256;
    int _i;

    address _addr;

    bool _b;
    bool _notConsB;

    bytes1 _bytes1;
    bytes8 _bytes8;
    bytes32 _bytes32;

    bytes1[] _b1Array;
    bytes8[] _b8Array;
    bytes32[] _b32Array;
    int[] _iArray;
    uint[] _uiArray;
    bool[] _bArray;

    string _str;
    string _constantStr;

    enum Order { First, Second, Third }
    Order order;

    event LogError(uint code, string message);
    event LogCreate(address contractAddr);
    
    function ParaType(){
        _addr = msg.sender;
        LogCreate(this);
    }
    
    function setUint(uint u, uint8 u8, uint16 u16, uint32 u32, uint256 u256){
        _u = u;
        _u8 = u8;
        _u16 = u16;
        _u32 = u32;
        _u256 = u256;
        LogError(0, 'setUint');
    }

    function getUint() constant returns(uint u, uint8 u8, uint16 u16, uint32 u32, uint256 u256){
        return(_u,_u8,_u16,_u32,_u256);
    }

    function setInt(int i, int8 i8, int16 i16, int32 i32, int256 i256){
        _i = i;
        _i8 = i8;
        _i16 = i16;
        _i32 = i32;
        _i256 = i256;
        LogError(0, 'setInt');
    }

    function getInt() constant returns(int i, int8 i8, int16 i16, int32 i32, int256 i256){
        return(_i,_i8,_i16,_i32,_i256);
    }

    function setBool(bool b){
        _b=b;
        LogError(0, 'setBool');
    }
    
    function setNotConstant(){
    _notConsB = _b;
    _constantStr = "NotConstant";
    LogError(0, 'setNotConstant');
    }
    
    function getNotConstant() returns (bool b,string constantStr){
       _notConsB = false;
       _constantStr = "NotConstant function";
        return (_notConsB,_constantStr);
    }
    
    function getConstant() constant returns (bool b,string constantStr){
        return (_notConsB,_constantStr);
    }

    function getBool() constant returns(bool b){
        return(_b);
    }

    function setByte(bytes1 b1, bytes8 b8, bytes32 b32){
        _bytes1 = b1;
        _bytes8 = b8;
        _bytes32 = b32;
        LogError(0, 'setByte');
    }

    function getByte() constant returns(bytes1 b1, bytes8 b8, bytes32 b32){
        return(_bytes1,_bytes8,_bytes32);
    }

    function setArray(bytes1[] b1, bytes8[] b8, bytes32[] b32,int[] iArray, uint[] uiArray,bool[] bArray){
        _b1Array = b1;
        _b8Array = b8;
        _b32Array = b32;
        _iArray = iArray;
        _uiArray = uiArray;
        _bArray = bArray;
        LogError(0, 'setArray');
    }

    function getArray() constant returns(bytes1[] b1, bytes8[] b8, bytes32[] b32,int[] iArray, uint[] uiArray,bool[] bArray){
        return(_b1Array,_b8Array,_b32Array,_iArray,_uiArray,_bArray);
    }

    function setString(string sData){
        _str = sData;
        LogError(0, 'setString');
    }

    function getString() constant returns(string sData){
        return(_str);
    }

    function setEnum(){
        order = Order.Second;
        LogError(0, 'setOrder');
    }

    function getEnum() constant returns(Order){
        return(order);
    }
}