# stable_memory_complex_struct

Check Vec by stable_memory

```bash
% dfx canister call backend add_from_default
()
% dfx canister call backend get_last_data --query
(
  record {
    value = record {
      id = "";
      result = record {
        ticks = vec {};
        tick_current = 0 : int32;
        liquidity = "";
        token0 = "";
        address = "";
        sqrt_ratio_x96 = "";
        tick_spacing = 0 : int32;
      };
      jsonrpc = "";
    };
    timestamp = 0 : nat64;
  },
)
% dfx canister call backend add_from_dummy
()
[~/repository/github.com/_linnefromice/linnefromice/stable_memory_complex_struct][20:57:11] % dfx canister call backend get_last_data --query
2024-01-10 11:57:12.540646 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] Panicked at 'called `Result::unwrap()` on an `Err` value: Custom(Cannot parse header 4449444c066c02f1fee18d0301d6a9bbae0a786c03dbb70171

Caused by:
    binary parser error: id at byte offset 25)', src/backend/src/types.rs:12:39
Error: Failed query call.
Caused by: Failed query call.
  The replica returned a replica error: reject code CanisterError, reject message IC0503: Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: Panicked at 'called `Result::unwrap()` on an `Err` value: Custom(Cannot parse header 4449444c066c02f1fee18d0301d6a9bbae0a786c03dbb70171

Caused by:
    binary parser error: id at byte offset 25)', src/backend/src/types.rs:12:39, error code Some("IC0503")
```

Check Cell by stable_memory

```bash
% dfx canister call backend get_cell --query
(
  record {
    value = record {
      id = "";
      result = record {
        ticks = vec {};
        tick_current = 0 : int32;
        liquidity = "";
        token0 = "";
        address = "";
        sqrt_ratio_x96 = "";
        tick_spacing = 0 : int32;
      };
      jsonrpc = "";
    };
    timestamp = 0 : nat64;
  },
)
% dfx canister call backend update_from_default
()
% dfx canister call backend get_cell --query
(
  record {
    value = record {
      id = "";
      result = record {
        ticks = vec {};
        tick_current = 0 : int32;
        liquidity = "";
        token0 = "";
        address = "";
        sqrt_ratio_x96 = "";
        tick_spacing = 0 : int32;
      };
      jsonrpc = "";
    };
    timestamp = 0 : nat64;
  },
)
% dfx canister call backend update_from_dummy
()
% dfx canister call backend get_cell --query
(
  record {
    value = record {
      id = "1";
      result = record {
        ticks = vec {};
        tick_current = -77_456 : int32;
        liquidity = "0x1d2f091ff09fb67174738";
        token0 = "0x6b175474e89094c44da98b954eedeac495271d0f";
        address = "0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8";
        sqrt_ratio_x96 = "0x55376cd2ad05b815780ecfb";
        tick_spacing = 60 : int32;
      };
      jsonrpc = "2.0";
    };
    timestamp = 1_704_887_709 : nat64;
  },
)
```

Supplement: data structure

```txt
% dfx canister call backend default --query 
(
  record {
    value = record {
      id = "";
      result = record {
        ticks = vec {};
        tick_current = 0 : int32;
        liquidity = "";
        token0 = "";
        address = "";
        sqrt_ratio_x96 = "";
        tick_spacing = 0 : int32;
      };
      jsonrpc = "";
    };
    timestamp = 0 : nat64;
  },
)
% dfx canister call backend dummy --query  
(
  record {
    value = record {
      id = "1";
      result = record {
        ticks = vec {};
        tick_current = -77_456 : int32;
        liquidity = "0x1d2f091ff09fb67174738";
        token0 = "0x6b175474e89094c44da98b954eedeac495271d0f";
        address = "0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8";
        sqrt_ratio_x96 = "0x55376cd2ad05b815780ecfb";
        tick_spacing = 60 : int32;
      };
      jsonrpc = "2.0";
    };
    timestamp = 1_704_887_914 : nat64;
  },
)
```
