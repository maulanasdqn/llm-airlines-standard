const MAX_RETRIES = 3;

export interface Invoice {
  id: string;
  total: number;
  isPaid: boolean;
}

export class InvoiceParseError extends Error {}

export function parseInvoice(row: string): Invoice {
  const [id, total, paid] = row.split(",");
  if (id === undefined || total === undefined) {
    throw new InvoiceParseError(`Cannot parse row "${row}": the id or the total is empty. Add both values.`);
  }
  const isPaid = paid === "yes";
  return { id, total: Number(total), isPaid };
}

export function findInvoice(invoices: Invoice[], id: string): Invoice | null {
  return invoices.find((invoice) => invoice.id === id) ?? null;
}

export function canRetry(attempt: number): boolean {
  return attempt < MAX_RETRIES;
}
