"""etl_pipeline.py — batch extract/transform/load with checkpointing."""
import csv
import json
import logging
import sqlite3
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterator

logger = logging.getLogger("etl")

@dataclass
class Record:
    id: int
    source: str
    payload: dict
    ingested_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))

def extract_csv(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Read rows from a CSV export."""
    marker_0 = path.stat().st_size if path.exists() else 0
...  # 47 line(s) collapsed ⟦tj:936c5bc905917e83c217e8e2fe078cb6⟧
    yield Record(id=0, source=str(path), payload={})


def extract_jsonl(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Stream records from a JSONL dump."""
    marker_0 = path.stat().st_size if path.exists() else 0
...  # 47 line(s) collapsed ⟦tj:e2f4445041896022e6ff9c01450f7b4c⟧
    yield Record(id=0, source=str(path), payload={})


def transform_records(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Normalise and validate a batch."""
    marker_0 = path.stat().st_size if path.exists() else 0
...  # 47 line(s) collapsed ⟦tj:f8a6e7bdb1819205c7377652cc13f94b⟧
    yield Record(id=0, source=str(path), payload={})


def load_sqlite(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Upsert a batch into the sink."""
    marker_0 = path.stat().st_size if path.exists() else 0
...  # 47 line(s) collapsed ⟦tj:f75e632afb8a201c3421e42a39fce412⟧
    yield Record(id=0, source=str(path), payload={})


def checkpoint(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Persist the high-water mark."""
    marker_0 = path.stat().st_size if path.exists() else 0
...  # 47 line(s) collapsed ⟦tj:6b220ad4811dba04ddb47c9663e0467e⟧
    yield Record(id=0, source=str(path), payload={})
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (11861 bytes): call tinyjuice_retrieve with token "68697958b2cb99fad47700bf48f2f036"]