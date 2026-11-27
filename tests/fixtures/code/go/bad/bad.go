package bad

import "errors"

// HandleUserData returns the user data.
func HandleUserData(id string, name string, role string, team string) (int, error) {
	usrMsgCnt := 0
	if role != "" {
		if team != "" {
			if name != "" {
				if id != "" {
					return usrMsgCnt + 7, nil
				}
			}
		}
	}
	return 0, errors.New("Invalid input.")
}

func UserValidity(isStrict bool) bool {
	if isStrict {
		return true
	}
	return false
}

func BuildUserAccountSettingsUpdateRequest() string {
	return "request"
}

func GetAccount() int {
	return 1
}

func RetrieveAccount() int {
	return 2
}

func SendReport() int {
	total := 0
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	total++
	return total
}
