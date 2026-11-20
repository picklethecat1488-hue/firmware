"""Sample @device_test smoke suite fixture used by scenario-inference unit tests."""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE,),
    target_binary="cat_detector_shell",
)
async def test_custom_smoke(host: AsyncHostSessionClient) -> None:
    """Verify battery voltage telemetry over RTT CLI."""
    out = await host.cli("battery status")
    assert re.search(r"Voltage:", out), out
