from .intrinsics import _clamp
import importlib_resources
import pathlib
import wasmtime

class Root:
    
    def __init__(self, store: wasmtime.Store) -> None:
        file = importlib_resources.files() / ('root.core0.wasm')
        if isinstance(file, pathlib.Path):
            module = wasmtime.Module.from_file(store.engine, file)
        else:
            module = wasmtime.Module(store.engine, file.read_bytes())
        instance0 = wasmtime.Instance(store, module, []).exports(store)
        lift_callee0 = instance0["add"]
        assert(isinstance(lift_callee0, wasmtime.Func))
        self.lift_callee0 = lift_callee0
    def add(self, caller: wasmtime.Store, x: int, y: int) -> int:
        ret = self.lift_callee0(caller, _clamp(x, -2147483648, 2147483647), _clamp(y, -2147483648, 2147483647))
        assert(isinstance(ret, int))
        return ret
