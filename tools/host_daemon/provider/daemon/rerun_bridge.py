"""Native `rerun-sdk` trace session streamer for `SENSOR` and `AUDIO` modes in host.py."""

from pathlib import Path
from typing import Any, Dict, Optional, Sequence
import uuid

MICROSECONDS_PER_SECOND: float = 1_000_000.0
MILLI_CELSIUS_PER_CELSIUS: float = 1_000.0
DEFAULT_RERUN_APP_ID: str = "xerxes_host_trace"


def _slugify_segment(raw: str) -> str:
    cleaned = "".join(ch.lower() if ch.isalnum() else "_" for ch in raw.strip())
    while "__" in cleaned:
        cleaned = cleaned.replace("__", "_")
    return cleaned.strip("_") or "channel"


def log_trace_events_to_rerun(rr_module: Any, events: Sequence[Dict[str, Any]]) -> int:
    """Log buffered `TraceSessionStore` items onto Rerun's `'timestamp'` duration timeline.

    Returns the number of Rerun entity log calls emitted.
    """
    logged = 0
    for item in events:
        ts_us = int(item.get("ts_us") or 0)
        rr_module.set_time("timestamp", duration=ts_us / MICROSECONDS_PER_SECOND)

        args = item.get("args") if isinstance(item.get("args"), dict) else {}
        rec_type = str(args.get("record_type") or "")
        category = str(item.get("category") or "")
        val = item.get("value")

        if rec_type == "Proximity":
            direction = _slugify_segment(str(args.get("direction") or "unknown"))
            is_valid = bool(args.get("valid", val is not None))
            state_label = "Valid" if is_valid else "Invalid"
            if isinstance(val, (int, float)):
                rr_module.log(f"proximity/{direction}/distance", rr_module.Scalars(float(val)))
                logged += 1
            rr_module.log(f"proximity/{direction}/state", rr_module.TextDocument(state_label))
            logged += 1
        elif rec_type == "Thermal":
            temp_mc = args.get("value", val)
            if isinstance(temp_mc, (int, float)):
                rr_module.log(
                    "thermal/temperature",
                    rr_module.Scalars(float(temp_mc) / MILLI_CELSIUS_PER_CELSIUS),
                )
                logged += 1
            overheating = bool(args.get("overheating", False))
            rr_module.log("thermal/overheating", rr_module.Scalars(1.0 if overheating else 0.0))
            logged += 1
        elif rec_type == "Gesture":
            gesture_name = str(args.get("gesture") or item.get("name") or "Gesture")
            rr_module.log("gesture", rr_module.TextDocument(gesture_name))
            logged += 1
        elif rec_type == "Battery":
            vol = args.get("voltage_mv", val)
            temp = args.get("temp_mc")
            state_str = str(args.get("state") or "")
            if isinstance(vol, (int, float)):
                rr_module.log("battery/voltage", rr_module.Scalars(float(vol)))
                logged += 1
            if isinstance(temp, (int, float)):
                rr_module.log("battery/temperature", rr_module.Scalars(float(temp)))
                logged += 1
            if state_str:
                rr_module.log("battery/state", rr_module.TextDocument(state_str))
                logged += 1
        elif rec_type == "FuelGauge":
            vol = args.get("voltage_mv", val)
            soc = args.get("soc_pct")
            if isinstance(vol, (int, float)):
                rr_module.log("fuel_gauge/voltage", rr_module.Scalars(float(vol)))
                logged += 1
            if isinstance(soc, (int, float)):
                rr_module.log("fuel_gauge/soc", rr_module.Scalars(float(soc)))
                logged += 1
        elif rec_type == "Motor":
            speed = args.get("speed", 0)
            running = bool(args.get("running", False))
            rr_module.log("motor/speed", rr_module.Scalars(float(speed)))
            rr_module.log("motor/running", rr_module.Scalars(1.0 if running else 0.0))
            logged += 2
            rpm = args.get("measured_rpm")
            if isinstance(rpm, (int, float)):
                rr_module.log("hall/rpm", rr_module.Scalars(float(rpm)))
                logged += 1
        elif category == "audio" or rec_type == "Audio":
            chan = _slugify_segment(str(item.get("track") or item.get("name") or "stream"))
            if isinstance(val, (int, float)):
                rr_module.log(f"audio/{chan}", rr_module.Scalars(float(val)))
                logged += 1
            else:
                rr_module.log(f"audio/{chan}", rr_module.TextDocument(str(item.get("name") or "audio_event")))
                logged += 1
        elif isinstance(val, (int, float)):
            track_slug = _slugify_segment(str(item.get("track") or item.get("name") or "metric"))
            rr_module.log(f"trace/{track_slug}", rr_module.Scalars(float(val)))
            logged += 1
        elif rec_type:
            rr_module.log(
                f"telemetry/{_slugify_segment(rec_type)}",
                rr_module.TextDocument(str(item.get("name") or rec_type)),
            )
            logged += 1

    return logged


def open_trace_in_rerun(
    events: Sequence[Dict[str, Any]],
    *,
    app_id: str = DEFAULT_RERUN_APP_ID,
    spawn: bool = True,
    save_rrd_path: Optional[Path] = None,
    rr_module: Any = None,
) -> Dict[str, Any]:
    """Stream `events` into the Rerun Viewer (or save `.rrd`) using `rerun-sdk`."""
    rr = rr_module
    if rr is None:
        try:
            import rerun as rr_imported
        except ImportError as exc:
            raise RuntimeError(
                "The 'rerun-sdk' Python package is required to open SENSOR/AUDIO traces in Rerun."
            ) from exc
        rr = rr_imported

    recording_id = str(uuid.uuid4())
    rr.init(app_id, recording_id=recording_id)
    if save_rrd_path is not None:
        save_rrd_path.parent.mkdir(parents=True, exist_ok=True)
        rr.save(str(save_rrd_path))
    if spawn:
        rr.spawn()

    logged_entities = log_trace_events_to_rerun(rr, events)
    return {
        "success": True,
        "viewer": "rerun",
        "app_id": app_id,
        "recording_id": recording_id,
        "event_count": len(events),
        "logged_entities": logged_entities,
        "rrd_path": str(save_rrd_path) if save_rrd_path is not None else None,
    }
