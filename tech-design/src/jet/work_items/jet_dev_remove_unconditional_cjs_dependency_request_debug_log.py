"""Quiet dependency-request diagnostics in the Jet Dev Server bounded context.

DDD framing:

- Bounded context: Dev Server.
- Application operation: serve one dependency-module request.
- Domain invariant: normal request serving is observationally quiet on stderr.
- Infrastructure policy: CJS classification is an implementation detail and
  may be surfaced only by a separately designed explicit diagnostic mode.

This work item removes the existing unconditional infrastructure leak. It does
not introduce a logging subsystem, a CLI flag, or a new public diagnostic
contract.

@spec #2405
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum

__aw_artifact_id__ = "artifact:jet-dev-server-dependency-serving-performance-and-quiet-normal-d/jet-dev-remove-unconditional-cjs-dependency-request-debug-log-wi-2405"
__aw_work_item__ = "2405"


class DependencyModuleKind(StrEnum):
    """Module kinds handled by the dependency-serving operation."""

    JAVASCRIPT = "js"
    COMMON_JS = "cjs"
    ECMASCRIPT_MODULE = "mjs"


@dataclass(frozen=True)
class DependencyRequest:
    """Value object for one dependency module entering the serving operation."""

    path: str
    module_kind: DependencyModuleKind
    classified_as_common_js: bool


def should_emit_default_classification_diagnostic(
    request: DependencyRequest,
) -> bool:
    """Keep normal dependency serving quiet regardless of CJS classification."""

    _ = request
    return False


def served_source_is_unchanged_by_diagnostic_policy(
    source_before_policy: str,
) -> str:
    """Diagnostics policy never changes the response or transform pipeline."""

    return source_before_policy


def design_contract() -> str:
    """Name the one implementation action and its non-expansion boundary."""

    return (
        "Remove the unconditional per-request CJS classification stderr write; "
        "preserve request classification, transformation, response bytes, and "
        "all existing startup/error diagnostics."
    )
