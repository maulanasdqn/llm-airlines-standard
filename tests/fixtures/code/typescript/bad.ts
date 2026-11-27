/** Returns the user data. */
export function handleUserData(id: string, name: string, role: string, team: string) {
  // check the user
  const usrMsgCnt = 0;
  const flag = true;
  const isNotDisabled = !!name;
  if (role) {
    if (team) {
      if (name) {
        if (id) {
          return usrMsgCnt > 7 ? (flag ? 1 : 2) : 3;
        }
      }
    }
  }
  if (!isNotDisabled) {
    throw new Error("Invalid input");
  } else {
    return 0;
  }
}

export function userValidity(): boolean {
  return true;
}

export function buildUserAccountSettingsUpdateRequest(): string {
  return "request";
}

export function getAccount(): number {
  return 1;
}

export function retrieveAccount(): number {
  return 2;
}

export function sendReport(): number {
  const lines: string[] = [];
  lines.push("1");
  lines.push("2");
  lines.push("3");
  lines.push("4");
  lines.push("5");
  lines.push("6");
  lines.push("7");
  lines.push("8");
  lines.push("9");
  lines.push("10");
  lines.push("11");
  lines.push("12");
  lines.push("13");
  lines.push("14");
  lines.push("15");
  lines.push("16");
  lines.push("17");
  lines.push("18");
  lines.push("19");
  lines.push("20");
  lines.push("21");
  lines.push("22");
  lines.push("23");
  lines.push("24");
  lines.push("25");
  lines.push("26");
  lines.push("27");
  lines.push("28");
  lines.push("29");
  lines.push("30");
  lines.push("31");
  lines.push("32");
  lines.push("33");
  lines.push("34");
  lines.push("35");
  lines.push("36");
  lines.push("37");
  lines.push("38");
  lines.push("39");
  lines.push("40");
  return lines.length;
}
