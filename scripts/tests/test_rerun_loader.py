import sys
import os
import types
import pytest

script_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "rerun-loader-csv"))

# Load extensionless script as a module
rerun_loader_csv = types.ModuleType("rerun_loader_csv")
rerun_loader_csv.__file__ = script_path
with open(script_path, "r", encoding="utf-8") as f:
    code = compile(f.read(), script_path, "exec")
    exec(code, rerun_loader_csv.__dict__)

sys.modules["rerun_loader_csv"] = rerun_loader_csv


def test_no_arguments(monkeypatch):
    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv"])

    with pytest.raises(SystemExit) as exc_info:
        rerun_loader_csv.main()
    assert exc_info.value.code == rerun_loader_csv.ERR_INCOMPATIBLE


def test_invalid_extension(monkeypatch):
    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv", "test.txt"])

    with pytest.raises(SystemExit) as exc_info:
        rerun_loader_csv.main()
    assert exc_info.value.code == rerun_loader_csv.ERR_INCOMPATIBLE


def test_invalid_header(tmp_path, monkeypatch):
    csv_file = tmp_path / "test.csv"
    csv_file.write_text("timestamp_us,record_type,invalid_header_field\n")

    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv", str(csv_file)])

    with pytest.raises(SystemExit) as exc_info:
        rerun_loader_csv.main()
    assert exc_info.value.code == rerun_loader_csv.ERR_INCOMPATIBLE


def test_valid_file_logs_to_rerun(tmp_path, monkeypatch):
    csv_file = tmp_path / "test.csv"
    csv_content = (
        "timestamp_us,record_type,val1,val2,val3,val4\n"
        "1000000,Battery,3800,25000,Ok,\n"
        "2000000,Motor,100,true,,\n"
        "3000000,Thermal,25000,false,,\n"
        "4000000,PeripheralError,I2CNackAddress,0x60,0x0E,\n"
    )
    csv_file.write_text(csv_content)

    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv", str(csv_file)])

    # Mock rerun library functions
    logged = []

    class MockRerun:
        def init(self, app_id, recording_id=None):
            pass

        def connect_grpc(self):
            pass

        def spawn(self):
            pass

        def stdout(self):
            pass

        def set_time(self, name, duration):
            pass

        def log(self, entity_path, entity):
            logged.append((entity_path, entity))

        class Scalars:
            def __init__(self, val):
                self.val = val

        class TextDocument:
            def __init__(self, val):
                self.val = val

    monkeypatch.setitem(sys.modules, "rerun", MockRerun())
    # Ensure sys.stdout is standard (non-TTY)
    monkeypatch.setattr(sys.stdout, "isatty", lambda: False)

    rerun_loader_csv.main()

    # Ensure something was logged!
    assert len(logged) > 0
    assert logged[0][0] == "battery/voltage"
    # Find logged paths
    paths = [item[0] for item in logged]
    assert "thermal/temperature" in paths
    assert "thermal/overheating" in paths
    assert "system/peripheral_error" in paths

    # Verify the formatted text document value for peripheral error
    idx = paths.index("system/peripheral_error")
    assert logged[idx][1].val == "I2CNackAddress (Addr: 0x60, Reg: 0x0E)"


def test_boot_warning_logged_to_rerun(tmp_path, monkeypatch):
    csv_file = tmp_path / "test.csv"
    csv_content = "timestamp_us,record_type,val1,val2,val3,val4\n2000000,Boot,Watchdog,,,\n"
    csv_file.write_text(csv_content)

    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv", str(csv_file)])

    logged = []

    class MockRerun:
        def init(self, app_id, recording_id=None):
            pass

        def connect_grpc(self):
            pass

        def spawn(self):
            pass

        def stdout(self):
            pass

        def set_time(self, name, duration):
            pass

        def log(self, entity_path, entity):
            logged.append((entity_path, entity))

        class Scalars:
            def __init__(self, val):
                self.val = val

        class TextDocument:
            def __init__(self, val):
                self.val = val

    monkeypatch.setitem(sys.modules, "rerun", MockRerun())
    monkeypatch.setattr(sys.stdout, "isatty", lambda: False)

    rerun_loader_csv.main()

    # Find logged paths
    paths = [item[0] for item in logged]
    assert "system/boot_reason" in paths
    assert "system/status" in paths

    idx_boot = paths.index("system/boot_reason")
    idx_status = paths.index("system/status")

    assert "🚨 SYSTEM RESET (Watchdog) 🚨" in logged[idx_boot][1].val
    assert "SYSTEM RESET: Watchdog" in logged[idx_status][1].val


def test_periodic_interval_logged_to_rerun(tmp_path, monkeypatch):
    csv_file = tmp_path / "test.csv"
    csv_content = (
        "timestamp_us,record_type,val1,val2,val3,val4\n"
        "2000000,PeriodicInterval,Battery,1000,,\n"
        "3000000,PeriodicInterval,Sensors,None,,\n"
    )
    csv_file.write_text(csv_content)

    monkeypatch.setattr(sys, "argv", ["rerun-loader-csv", str(csv_file)])

    logged = []

    class MockRerun:
        def init(self, app_id, recording_id=None):
            pass

        def connect_grpc(self):
            pass

        def spawn(self):
            pass

        def stdout(self):
            pass

        def set_time(self, name, duration):
            pass

        def log(self, entity_path, entity):
            logged.append((entity_path, entity))

        class Scalars:
            def __init__(self, val):
                self.val = val

        class TextDocument:
            def __init__(self, val):
                self.val = val

    monkeypatch.setitem(sys.modules, "rerun", MockRerun())
    monkeypatch.setattr(sys.stdout, "isatty", lambda: False)

    rerun_loader_csv.main()

    # Find logged paths
    paths = [item[0] for item in logged]
    assert "periodic_interval/battery" in paths
    assert "periodic_interval/sensors" in paths

    idx_battery = paths.index("periodic_interval/battery")
    idx_sensors = paths.index("periodic_interval/sensors")

    assert logged[idx_battery][1].val == 1000.0
    assert logged[idx_sensors][1].val == 0.0
