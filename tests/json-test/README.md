# Json Test

Test CITA using [Tests](https://github.com/citahub/cita-testdata/)

## Usage

### State Tests

```sh
$ cd cita

$ cargo test --features sha3hash state_test::tests::test_json_state
```

### VM Tests

```sh
$ cd cita

$ cargo test vm_test::tests::test_json_vm
```
