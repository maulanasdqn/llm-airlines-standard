package good

import (
	"fmt"
	"strings"
)

const (
	maxRetries = 3
	idField    = 0
	totalField = 1
	paidField  = 2
)

type Invoice struct {
	ID     string
	Total  string
	IsPaid bool
}

type ParseError struct {
	Row string
}

func (parseError *ParseError) Error() string {
	return fmt.Sprintf("parse row %q: the id or the total is empty: add both values", parseError.Row)
}

func ParseInvoice(row string) (Invoice, error) {
	fields := strings.Split(row, ",")
	if len(fields) <= totalField || fields[idField] == "" || fields[totalField] == "" {
		return Invoice{}, &ParseError{Row: row}
	}
	isPaid := len(fields) > paidField && fields[paidField] == "yes"
	return Invoice{ID: fields[idField], Total: fields[totalField], IsPaid: isPaid}, nil
}

func CanRetry(attempt int) bool {
	return attempt < maxRetries
}
