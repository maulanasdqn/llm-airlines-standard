# Code examples

We wrote every example on this page to show a rule.
The examples use TypeScript. The rules are the same in each language.

## Names

| Rule | Before | After |
|---|---|---|
| C1.3 | `getUser`, `retrieveOrder`, `obtainInvoice` | `getUser`, `getOrder`, `getInvoice` |
| C1.4 | `function email(user)` that sends an email | `function sendEmail(user)` |
| C1.6 | `handleInvoiceData()` | `parseInvoiceRows()` |
| C1.7 | `const usrMsgCnt = 0` | `const userMessageCount = 0` |
| C1.8 | `timeout: 30` | `timeoutSeconds: 30` |
| C2.1 | `userAccountSettingsUpdateRequest` | `settings.UpdateRequest` |
| C2.3 | `invoice.invoiceTotal` | `invoice.total` |
| C3.3 | `event: "orderCancel"` | `event: "orderCanceled"` |
| C4.4 | `isNotDisabled` | `isEnabled` |

## Replace a comment with code (C6.1, C6.4)

Before:

```ts
// 30 days in ms
const t = 2592000000;

// skip users that are not active or that have no email
if (!u.active || !u.email) {
  return;
}
```

After:

```ts
const TRIAL_LENGTH_MS = 30 * 24 * 60 * 60 * 1000;

const canReceiveEmail = user.isActive && user.email !== null;
if (!canReceiveEmail) {
  return;
}
```

## Put the risk in the name (C7.1, C7.2)

Before:

```ts
// WARNING: deletes everything, there is no undo
export async function cleanup(bucket: string): Promise<void> {
  await storage.deleteObjects(bucket, "*");
}
```

After:

```ts
export async function deleteAllObjects(
  bucket: string,
  options: { isConfirmed: boolean } = { isConfirmed: false },
): Promise<void> {
  if (!options.isConfirmed) {
    throw new UnconfirmedDeleteError(
      `Cannot delete the objects in "${bucket}": the delete is not confirmed. Pass { isConfirmed: true }.`,
    );
  }
  await storage.deleteObjects(bucket, "*");
}
```

## Put the condition first (C5.2)

Before:

```ts
function sendReminder(invoice: Invoice): void {
  if (invoice.isOverdue) {
    if (invoice.customer.email !== null) {
      mailer.send(invoice.customer.email, buildReminder(invoice));
    }
  }
}
```

After:

```ts
function sendReminder(invoice: Invoice): void {
  if (!invoice.isOverdue) {
    return;
  }
  const email = invoice.customer.email;
  if (email === null) {
    return;
  }
  mailer.send(email, buildReminder(invoice));
}
```

## Show the failure mode in the signature (C6.5)

Before:

```ts
/** Returns the user. Throws if the user does not exist. */
function getUser(id: string): User {
```

After:

```ts
function findUser(id: UserId): User | null {
```

The verb `find` says that the result can be nothing (C3.2). The return type says the same thing.
