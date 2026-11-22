"""Strongly typed compile-time & runtime tracing modes and external viewer mappings for host.py."""

from enum import Enum
import time
from typing import Any, Dict, List, Optional, Tuple

PERFETTO_UI_ORIGIN: str = "https://ui.perfetto.dev"
DEFAULT_LOCAL_DAEMON_HOST: str = "127.0.0.1"
DEFAULT_DAEMON_HTTP_PORT: int = 8899
PERFETTO_BRIDGE_ROUTE: str = "/perfetto"
PERFETTO_TRACE_API_PATH: str = "/api/trace/perfetto.json"


class TraceMode(str, Enum):
    """Supported target firmware tracing modes selectable from Target Config and CLI."""

    NONE = "NONE"
    SENSOR = "SENSOR"
    PERF = "PERF"
    AUDIO = "AUDIO"


class ExternalTraceViewer(str, Enum):
    """External trace visualization backend paired with each `TraceMode`."""

    NONE = "none"
    PERFETTO = "perfetto"
    RERUN = "rerun"


TRACE_MODE_CARGO_FEATURES: Dict[TraceMode, Tuple[str, ...]] = {
    TraceMode.NONE: (),
    TraceMode.SENSOR: ("trace-sensors",),
    TraceMode.PERF: ("trace-perf",),
    TraceMode.AUDIO: ("trace-audio",),
}

TRACE_MODE_EXTERNAL_VIEWER: Dict[TraceMode, ExternalTraceViewer] = {
    TraceMode.NONE: ExternalTraceViewer.NONE,
    TraceMode.SENSOR: ExternalTraceViewer.RERUN,
    TraceMode.PERF: ExternalTraceViewer.PERFETTO,
    TraceMode.AUDIO: ExternalTraceViewer.RERUN,
}

TRACE_MODE_DESCRIPTORS: Tuple[Dict[str, Any], ...] = (
    {
        "mode": TraceMode.NONE.value,
        "label": "NONE",
        "summary": "Standard defmt logs; zero runtime trace overhead",
        "cargo_features": [],
        "external_viewer": ExternalTraceViewer.NONE.value,
        "enabled_in_ui": True,
        "badge": "default",
    },
    {
        "mode": TraceMode.SENSOR.value,
        "label": "SENSOR",
        "summary": "Sensor reports & measured values only • External viewer: Rerun",
        "cargo_features": list(TRACE_MODE_CARGO_FEATURES[TraceMode.SENSOR]),
        "external_viewer": ExternalTraceViewer.RERUN.value,
        "enabled_in_ui": True,
        "badge": "trace-sensors",
    },
    {
        "mode": TraceMode.PERF.value,
        "label": "PERF",
        "summary": "All CPU spans, CPU usage % & system telemetry (excl. sensors) • Viewer: Perfetto",
        "cargo_features": list(TRACE_MODE_CARGO_FEATURES[TraceMode.PERF]),
        "external_viewer": ExternalTraceViewer.PERFETTO.value,
        "enabled_in_ui": True,
        "badge": "trace-perf",
    },
    {
        "mode": TraceMode.AUDIO.value,
        "label": "AUDIO",
        "summary": "PDM/I2S audio stream & DSP frame tracing • External viewer: Rerun",
        "cargo_features": list(TRACE_MODE_CARGO_FEATURES[TraceMode.AUDIO]),
        "external_viewer": ExternalTraceViewer.RERUN.value,
        "enabled_in_ui": False,
        "badge": "PLANNED",
    },
)


def normalize_trace_mode(raw: Optional[str]) -> TraceMode:
    """Normalize a case-insensitive trace mode string into a [`TraceMode`] enum variant."""
    if raw is None:
        return TraceMode.NONE
    cleaned = str(raw).strip().upper()
    if not cleaned:
        return TraceMode.NONE
    for variant in TraceMode:
        if variant.value == cleaned:
            return variant
    valid = ", ".join(m.value.lower() for m in TraceMode)
    raise ValueError(f"Unsupported trace mode '{raw}'. Expected one of: {valid}")


def get_cargo_feature_args(mode: TraceMode) -> List[str]:
    """Return Cargo `--features <list>` CLI arguments for `mode` (empty for `TraceMode.NONE`)."""
    features = TRACE_MODE_CARGO_FEATURES.get(mode, ())
    if not features:
        return []
    return ["--features", ",".join(features)]


def get_external_trace_viewer(mode: TraceMode) -> ExternalTraceViewer:
    """Return the canonical [`ExternalTraceViewer`] (`perfetto` for `PERF`, `rerun` for `SENSOR`/`AUDIO`)."""
    return TRACE_MODE_EXTERNAL_VIEWER.get(mode, ExternalTraceViewer.NONE)


def build_perfetto_bridge_url(
    daemon_host: str = DEFAULT_LOCAL_DAEMON_HOST,
    daemon_port: int = DEFAULT_DAEMON_HTTP_PORT,
    cache_bust_ms: Optional[int] = None,
) -> Tuple[str, str]:
    """Build `(trace_url, bridge_url)` for the same-origin `/perfetto` `<iframe>` + `postMessage` bridge."""
    clean_host = DEFAULT_LOCAL_DAEMON_HOST if daemon_host in ("", "0.0.0.0", "::") else str(daemon_host)
    ts_ms = int(time.time() * 1000) if cache_bust_ms is None else int(cache_bust_ms)
    trace_url = f"{PERFETTO_TRACE_API_PATH}?ts={ts_ms}"
    bridge_url = f"http://{clean_host}:{int(daemon_port)}{PERFETTO_BRIDGE_ROUTE}?ts={ts_ms}"
    return trace_url, bridge_url
