from __future__ import annotations

from dataclasses import dataclass

MAX_RETRIES = 3


class InvoiceParseError(ValueError):
    pass


@dataclass(frozen=True)
class Invoice:
    invoice_id: str
    total: float
    is_paid: bool


def parse_invoice(row: str) -> Invoice:
    invoice_id, total, paid = [*row.split(","), "", ""][:3]
    if not invoice_id or not total:
        message = f'Cannot parse row "{row}": the id or the total is empty. Add both values.'
        raise InvoiceParseError(message)
    return Invoice(invoice_id=invoice_id, total=float(total), is_paid=paid == "yes")


def find_invoice(invoices: list[Invoice], invoice_id: str) -> Invoice | None:
    return next((invoice for invoice in invoices if invoice.invoice_id == invoice_id), None)


def can_retry(attempt: int) -> bool:
    return attempt < MAX_RETRIES
