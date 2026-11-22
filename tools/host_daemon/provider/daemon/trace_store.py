"""Thread-safe live trace session ring buffer, span pairing engine, and CSV/Perfetto exporters."""

from collections import deque
import csv
import io
import json
from pathlib import Path
import threading
from typing import Any, Deque, Dict, List, Optional, Tuple

MAX_TRACE_EVENTS: int = 50_000
DEFAULT_TRACE_EXPORT_SUBDIR: str = "recordings/traces"
CORE0_TID: int = 1
CORE1_TID: int = 2
SYSTEM_TELEMETRY_TID: int = 3
DYNAMIC_TRACK_BASE_TID: int = 10

CSV_HEADER_COLUMNS: Tuple[str, ...] = (
    "timestamp_us",
    "track",
    "phase",
    "category",
    "name",
    "duration_us",
    "value",
    "unit",
    "record_type",
    "val1",
    "val2",
    "val3",
    "val4",
)


def _is_root_parent_id(parent_id: Optional[str]) -> bool:
    """Return `True` when `parent_id` is absent or denotes an all-zero root span ID."""
    if not parent_id:
        return True
    stripped = parent_id.strip()
    return not stripped or all(ch == "0" for ch in stripped)


def _is_span_name_match(open_name: str, candidate_name: str) -> bool:
    """Match span names ignoring trailing `(param=...)` formatting suffixes and `module::` prefixes."""
    if not open_name or not candidate_name:
        return False
    base_open = open_name.split("(", 1)[0].strip()
    base_cand = candidate_name.split("(", 1)[0].strip()
    if base_open == base_cand:
        return True
    return base_open.endswith(f"::{base_cand}") or base_cand.endswith(f"::{base_open}")


class TraceSessionStore:
    """Thread-safe store pairing streaming `B`/`E` spans and buffering counter/sensor trace events."""

    def __init__(self, max_events: int = MAX_TRACE_EVENTS) -> None:
        """Initialize an empty recording trace session store."""
        self.max_events = max(100, int(max_events))
        self._lock = threading.Lock()
        self._recording: bool = True
        self._session_epoch: int = 1
        self._next_id: int = 0
        self._next_seq: int = 0
        self._items: Deque[Dict[str, Any]] = deque()
        self._items_by_id: Dict[int, Dict[str, Any]] = {}
        self._open_spans_by_track: Dict[str, List[int]] = {}
        self._min_ts_us: Optional[int] = None
        self._max_ts_us: Optional[int] = None

    @property
    def recording(self) -> bool:
        """Return `True` when live RTT trace events are actively being recorded."""
        with self._lock:
            return self._recording

    @property
    def session_epoch(self) -> int:
        """Return the current monotonically increasing trace session epoch."""
        with self._lock:
            return self._session_epoch

    @property
    def event_count(self) -> int:
        """Return the number of currently buffered trace items."""
        with self._lock:
            return len(self._items)

    def set_recording(self, active: bool) -> bool:
        """Pause (`False`) or resume (`True`) live trace capture and return the new state."""
        with self._lock:
            self._recording = bool(active)
            return self._recording

    def clear(self) -> None:
        """Purge all buffered trace events, open span stacks, and time bounds, advancing `session_epoch`."""
        with self._lock:
            self._items.clear()
            self._items_by_id.clear()
            self._open_spans_by_track.clear()
            self._min_ts_us = None
            self._max_ts_us = None
            self._next_id = 0
            self._next_seq = 0
            self._session_epoch += 1

    def _evict_oldest_unlocked(self) -> None:
        while len(self._items) >= self.max_events:
            oldest = self._items.popleft()
            oldest_id = int(oldest["id"])
            self._items_by_id.pop(oldest_id, None)
            track_stack = self._open_spans_by_track.get(str(oldest["track"]))
            if track_stack and oldest_id in track_stack:
                track_stack.remove(oldest_id)

    def _update_bounds_unlocked(self, ts_us: int) -> None:
        if self._min_ts_us is None or ts_us < self._min_ts_us:
            self._min_ts_us = ts_us
        if self._max_ts_us is None or ts_us > self._max_ts_us:
            self._max_ts_us = ts_us

    def ingest_event(self, ev_obj: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Ingest a structured `trace_event` dict from `host_cli --json-events`."""
        raw_ts = ev_obj.get("ts_us", 0)
        ts_us = max(0, int(raw_ts)) if isinstance(raw_ts, (int, float)) and not isinstance(raw_ts, bool) else 0
        track = str(ev_obj.get("track") or "Device Logs & Telemetry")
        ph = str(ev_obj.get("ph") or "i")
        category = str(ev_obj.get("category") or "telemetry")
        name = str(ev_obj.get("name") or track)
        span_id = str(ev_obj["span_id"]) if ev_obj.get("span_id") is not None else None
        parent_id = str(ev_obj["parent_id"]) if ev_obj.get("parent_id") is not None else None
        module = str(ev_obj["module"]) if ev_obj.get("module") is not None else None
        raw_val = ev_obj.get("value")
        value = float(raw_val) if isinstance(raw_val, (int, float)) and not isinstance(raw_val, bool) else None
        unit = str(ev_obj["unit"]) if ev_obj.get("unit") is not None else None
        args_raw = ev_obj.get("args")
        args: Dict[str, Any] = dict(args_raw) if isinstance(args_raw, dict) else {}

        with self._lock:
            if not self._recording:
                return None

            self._update_bounds_unlocked(ts_us)

            if ph == "E":
                stack = self._open_spans_by_track.get(track)
                if stack:
                    matched_pos: Optional[int] = None
                    if span_id is not None:
                        for idx in range(len(stack) - 1, -1, -1):
                            cand = self._items_by_id.get(stack[idx])
                            if cand is not None and cand.get("span_id") == span_id:
                                matched_pos = idx
                                break
                    if matched_pos is None:
                        for idx in range(len(stack) - 1, -1, -1):
                            cand = self._items_by_id.get(stack[idx])
                            if cand is not None and (
                                cand.get("name") == name or _is_span_name_match(str(cand.get("name") or ""), name)
                            ):
                                matched_pos = idx
                                break
                    if matched_pos is None and span_id is None and not name:
                        matched_pos = len(stack) - 1
                    if matched_pos is None:
                        return None

                    # Implicitly close any nested child spans above `matched_pos`
                    while len(stack) > matched_pos:
                        popped_id = stack.pop()
                        open_item = self._items_by_id.get(popped_id)
                        if open_item is not None:
                            start_ts = int(open_item["ts_us"])
                            end_ts = max(start_ts, ts_us)
                            self._next_seq += 1
                            open_item["ph"] = "X"
                            open_item["end_ts_us"] = end_ts
                            open_item["dur_us"] = end_ts - start_ts
                            open_item["seq"] = self._next_seq
                    matched_item = self._items_by_id.get(popped_id) if "popped_id" in locals() else None
                    return dict(matched_item) if matched_item is not None else None
                return None

            if ph == "B":
                stack = self._open_spans_by_track.setdefault(track, [])
                if stack:
                    unwind_pos: Optional[int] = None
                    if _is_root_parent_id(parent_id) and ev_obj.get("parent_id") is not None:
                        unwind_pos = 0
                    else:
                        for idx, open_id in enumerate(stack):
                            cand = self._items_by_id.get(open_id)
                            if cand is None:
                                continue
                            cand_span_id = cand.get("span_id")
                            cand_name = str(cand.get("name") or "")
                            if (
                                (span_id is not None and cand_span_id == span_id)
                                or cand_name == name
                                or _is_span_name_match(cand_name, name)
                            ):
                                unwind_pos = idx
                                break
                        if unwind_pos is None and parent_id and not _is_root_parent_id(parent_id):
                            for idx, open_id in enumerate(stack):
                                cand = self._items_by_id.get(open_id)
                                if cand is not None and cand.get("span_id") == parent_id:
                                    if idx + 1 < len(stack):
                                        unwind_pos = idx + 1
                                    break
                    if unwind_pos is not None:
                        while len(stack) > unwind_pos:
                            popped_id = stack.pop()
                            open_item = self._items_by_id.get(popped_id)
                            if open_item is not None:
                                start_ts = int(open_item["ts_us"])
                                end_ts = max(start_ts, ts_us)
                                self._next_seq += 1
                                open_item["ph"] = "X"
                                open_item["end_ts_us"] = end_ts
                                open_item["dur_us"] = end_ts - start_ts
                                open_item["seq"] = self._next_seq

            self._evict_oldest_unlocked()
            self._next_id += 1
            self._next_seq += 1
            item_id = self._next_id
            seq = self._next_seq

            depth = 0
            if ph == "B":
                stack = self._open_spans_by_track.setdefault(track, [])
                depth = len(stack)
                stack.append(item_id)

            item: Dict[str, Any] = {
                "id": item_id,
                "seq": seq,
                "ts_us": ts_us,
                "end_ts_us": ts_us,
                "dur_us": 0,
                "depth": depth,
                "track": track,
                "ph": ph,
                "category": category,
                "name": name,
                "span_id": span_id,
                "parent_id": parent_id,
                "module": module,
                "value": value,
                "unit": unit,
                "args": args,
            }
            self._items.append(item)
            self._items_by_id[item_id] = item
            return dict(item)

    def get_events(
        self,
        after_seq: int = 0,
        limit: int = 10_000,
        session_epoch: Optional[int] = None,
    ) -> Dict[str, Any]:
        """Return incremental trace events (`seq > after_seq`), track summaries, and session metadata."""
        capped_limit = max(1, min(int(limit), self.max_events))
        with self._lock:
            epoch_mismatch = session_epoch is not None and int(session_epoch) != self._session_epoch
            effective_after_seq = 0 if (epoch_mismatch or after_seq > self._next_seq) else after_seq
            changed = [dict(it) for it in self._items if int(it["seq"]) > effective_after_seq]
            if len(changed) > capped_limit:
                changed = changed[-capped_limit:]

            tracks_summary: Dict[str, Dict[str, Any]] = {}
            for it in self._items:
                t_name = str(it["track"])
                summary = tracks_summary.get(t_name)
                if summary is None:
                    summary = {
                        "name": t_name,
                        "category": it["category"],
                        "unit": it["unit"],
                        "count": 0,
                        "max_depth": 0,
                        "min_val": None,
                        "max_val": None,
                    }
                    tracks_summary[t_name] = summary
                summary["count"] += 1
                if int(it["depth"]) > int(summary["max_depth"]):
                    summary["max_depth"] = int(it["depth"])
                val = it.get("value")
                if isinstance(val, (int, float)):
                    if summary["min_val"] is None or val < summary["min_val"]:
                        summary["min_val"] = val
                    if summary["max_val"] is None or val > summary["max_val"]:
                        summary["max_val"] = val

            return {
                "recording": self._recording,
                "session_epoch": self._session_epoch,
                "cursor": self._next_seq,
                "total_events": len(self._items),
                "min_ts_us": self._min_ts_us or 0,
                "max_ts_us": self._max_ts_us or 0,
                "tracks": list(tracks_summary.values()),
                "events": changed,
            }

    def snapshot_all(self) -> List[Dict[str, Any]]:
        """Return a thread-safe chronological copy of all buffered trace items."""
        with self._lock:
            items = [dict(it) for it in self._items]
        items.sort(key=lambda entry: (int(entry["ts_us"]), int(entry["id"])))
        return items

    def format_csv(self) -> str:
        """Serialize all buffered trace items into canonical CSV format."""
        items = self.snapshot_all()
        buf = io.StringIO()
        writer = csv.writer(buf, lineterminator="\n")
        writer.writerow(CSV_HEADER_COLUMNS)

        for it in items:
            args = it.get("args") if isinstance(it.get("args"), dict) else {}
            rec_type = str(args.get("record_type") or it.get("category") or "")
            csv_vals = args.get("csv") if isinstance(args.get("csv"), list) else []
            cols = [str(csv_vals[idx]) if idx < len(csv_vals) and csv_vals[idx] is not None else "" for idx in range(4)]
            val = it.get("value")
            val_str = "" if val is None else (str(int(val)) if float(val).is_integer() else f"{val}")
            dur_val = int(it.get("dur_us") or 0)
            dur_str = str(dur_val) if it.get("ph") in ("X", "B") else ""
            writer.writerow(
                [
                    int(it["ts_us"]),
                    it["track"],
                    it["ph"],
                    it["category"],
                    it["name"],
                    dur_str,
                    val_str,
                    it.get("unit") or "",
                    rec_type,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                ]
            )
        return buf.getvalue()

    def format_perfetto_json(self, chip: Optional[str] = None) -> str:
        """Serialize all buffered trace items into Chrome / Perfetto JSON Trace Event Format."""
        items = self.snapshot_all()
        proc_name = f"{(chip or 'TARGET').upper()} MCU"
        trace_events: List[Dict[str, Any]] = [
            {"name": "process_name", "ph": "M", "pid": 1, "args": {"name": proc_name}},
            {"name": "thread_name", "ph": "M", "pid": 1, "tid": CORE0_TID, "args": {"name": "CPU Core 0"}},
            {"name": "thread_name", "ph": "M", "pid": 1, "tid": CORE1_TID, "args": {"name": "CPU Core 1"}},
            {
                "name": "thread_name",
                "ph": "M",
                "pid": 1,
                "tid": SYSTEM_TELEMETRY_TID,
                "args": {"name": "System & Power Events"},
            },
        ]
        track_tids: Dict[str, int] = {
            "CPU Core 0": CORE0_TID,
            "CPU Core 1": CORE1_TID,
            "System & Power Events": SYSTEM_TELEMETRY_TID,
        }
        next_tid = DYNAMIC_TRACK_BASE_TID

        for it in items:
            track = str(it["track"])
            ph = str(it["ph"])
            ts_us = int(it["ts_us"])
            if ph == "C":
                trace_events.append(
                    {
                        "cat": it["category"],
                        "name": track,
                        "ph": "C",
                        "pid": 1,
                        "ts": ts_us,
                        "args": {
                            "value": it["value"] if it["value"] is not None else 0.0,
                            "unit": it.get("unit"),
                        },
                    }
                )
                continue

            tid = track_tids.get(track)
            if tid is None:
                tid = next_tid
                next_tid += 1
                track_tids[track] = tid
                trace_events.append({"name": "thread_name", "ph": "M", "pid": 1, "tid": tid, "args": {"name": track}})

            ev_args = dict(it.get("args") or {})
            if it.get("module"):
                ev_args["module"] = it["module"]
            if it.get("unit"):
                ev_args["unit"] = it["unit"]
            if it.get("value") is not None:
                ev_args.setdefault("value", it["value"])

            if ph == "X":
                trace_events.append(
                    {
                        "cat": it["category"],
                        "name": it["name"],
                        "ph": "X",
                        "pid": 1,
                        "tid": tid,
                        "ts": ts_us,
                        "dur": max(1, int(it.get("dur_us") or 0)),
                        "args": ev_args,
                    }
                )
            elif ph == "B":
                # Open in-flight span: export up to session max timestamp as a completed slice
                end_ts = max(ts_us + 1, int(self._max_ts_us or ts_us + 1))
                ev_args["in_flight"] = True
                trace_events.append(
                    {
                        "cat": it["category"],
                        "name": it["name"],
                        "ph": "X",
                        "pid": 1,
                        "tid": tid,
                        "ts": ts_us,
                        "dur": end_ts - ts_us,
                        "args": ev_args,
                    }
                )
            else:
                trace_events.append(
                    {
                        "cat": it["category"],
                        "name": it["name"],
                        "ph": "i",
                        "s": "t",
                        "pid": 1,
                        "tid": tid,
                        "ts": ts_us,
                        "args": ev_args,
                    }
                )

        return json.dumps({"traceEvents": trace_events, "displayTimeUnit": "ms"}, indent=2)

    def export_to_disk(
        self,
        repo_root: Path,
        fmt: str,
        dest_path_raw: Optional[str] = None,
        *,
        chip: Optional[str] = None,
        default_stem: str = "target_trace",
    ) -> Dict[str, Any]:
        """Write buffered trace session to disk in `'csv'` or `'perfetto'` (`.json`) format."""
        cleaned_fmt = str(fmt or "").strip().lower()
        if cleaned_fmt not in ("csv", "perfetto", "json"):
            raise ValueError(f"Unsupported trace export format '{fmt}'. Expected 'csv' or 'perfetto'.")

        is_csv = cleaned_fmt == "csv"
        ext = ".csv" if is_csv else ".perfetto.json"
        if dest_path_raw and str(dest_path_raw).strip():
            raw_p = Path(str(dest_path_raw).strip())
            target_path = raw_p if raw_p.is_absolute() else (repo_root / raw_p)
            if is_csv and target_path.suffix.lower() != ".csv":
                target_path = target_path.with_name(f"{target_path.name}.csv")
            elif not is_csv and not str(target_path.name).lower().endswith(".json"):
                target_path = target_path.with_name(f"{target_path.name}.perfetto.json")
        else:
            target_path = repo_root / DEFAULT_TRACE_EXPORT_SUBDIR / f"{default_stem}{ext}"

        content = self.format_csv() if is_csv else self.format_perfetto_json(chip=chip)
        event_count = len(self.snapshot_all())
        target_path.parent.mkdir(parents=True, exist_ok=True)
        encoded = content.encode("utf-8")
        target_path.write_bytes(encoded)
        return {
            "success": True,
            "format": "csv" if is_csv else "perfetto",
            "path": str(target_path),
            "event_count": event_count,
            "bytes": len(encoded),
        }
