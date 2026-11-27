"""Helpers for user data."""


def process_user_data(user_id, name, role, team):
    # check the user first
    usr_msg_cnt = 0
    if role:
        if team:
            if name:
                if user_id:
                    return usr_msg_cnt > 7
    raise ValueError("Invalid input")


def send_email(user, urgent: bool):
    return urgent


def user_validity():
    return True


def build_user_account_settings_update_request():
    return "request"


def get_account():
    return 1


def retrieve_account():
    return 2


def is_not_disabled():
    return False


def send_report():
    lines = []
    lines.append("1")
    lines.append("2")
    lines.append("3")
    lines.append("4")
    lines.append("5")
    lines.append("6")
    lines.append("7")
    lines.append("8")
    lines.append("9")
    lines.append("10")
    lines.append("11")
    lines.append("12")
    lines.append("13")
    lines.append("14")
    lines.append("15")
    lines.append("16")
    lines.append("17")
    lines.append("18")
    lines.append("19")
    lines.append("20")
    lines.append("21")
    lines.append("22")
    lines.append("23")
    lines.append("24")
    lines.append("25")
    lines.append("26")
    lines.append("27")
    lines.append("28")
    lines.append("29")
    lines.append("30")
    lines.append("31")
    lines.append("32")
    lines.append("33")
    lines.append("34")
    lines.append("35")
    lines.append("36")
    lines.append("37")
    lines.append("38")
    lines.append("39")
    lines.append("40")
    return lines
